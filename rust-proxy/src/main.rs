//! Week 1: pass-through reverse proxy. SSE response chunks are tapped: each is copied into
//! a fixed window and sent through mojo_inspect_window; the verdict is logged. Chunks are
//! forwarded UNMODIFIED this week. Upstream is plain HTTP (vLLM / mock); TLS (rustls) is
//! not wired yet.
mod ffi;
mod policy;

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{body::Incoming, header, service::service_fn, Request, Response, StatusCode, Uri};
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::{TokioExecutor, TokioIo},
};
use std::{convert::Infallible, env, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

type Resp = Response<BoxBody<Bytes, hyper::Error>>;
const WINDOW: usize = 1024;

struct State {
    upstream: String,
    client: Client<HttpConnector, Incoming>,
    policy: Arc<policy::Policy>,
}

fn error_resp(code: StatusCode, msg: &'static str) -> Resp {
    let body = Full::new(Bytes::from_static(msg.as_bytes())).map_err(|n| match n {}).boxed();
    let mut r = Response::new(body);
    *r.status_mut() = code;
    r
}

async fn handle(req: Request<Incoming>, st: Arc<State>) -> Result<Resp, Infallible> {
    let pq = req.uri().path_and_query().map(|p| p.as_str()).unwrap_or("/");
    let uri: Uri = match format!("{}{}", st.upstream, pq).parse() {
        Ok(u) => u,
        Err(_) => return Ok(error_resp(StatusCode::BAD_REQUEST, "bad uri")),
    };
    let (mut parts, body) = req.into_parts();
    parts.uri = uri;
    parts.headers.remove(header::HOST);
    let resp = match st.client.request(Request::from_parts(parts, body)).await {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!("upstream error: {e}");
            return Ok(error_resp(StatusCode::BAD_GATEWAY, "upstream error"));
        }
    };
    let (rp, rb) = resp.into_parts();
    let is_sse = rp
        .headers
        .get(header::CONTENT_TYPE)
        .map(|v| v.as_bytes().starts_with(b"text/event-stream"))
        .unwrap_or(false);
    let body = if is_sse {
        let policy = st.policy.clone();
        rb.map_frame(move |frame| {
            if let Some(data) = frame.data_ref() {
                // Week 2 replaces this copy with the slab-backed in-place lookahead window
                // and delta.content extraction.
                let mut win = [0u8; WINDOW];
                let n = data.len().min(WINDOW);
                win[..n].copy_from_slice(&data[..n]);
                let r = ffi::inspect(&mut win[..n], 0, policy.as_bytes());
                if r.verdict == ffi::VERDICT_PASS {
                    tracing::info!(bytes = n, ns = r.elapsed_ns, "VERDICT_PASS");
                } else {
                    tracing::warn!(verdict = r.verdict, rule = r.rule_id, score = r.score, "non-pass verdict (ignored in week 1)");
                }
            }
            frame
        })
        .boxed()
    } else {
        rb.boxed()
    };
    Ok(Response::from_parts(rp, body))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();
    let listen: SocketAddr = env::var("AEGIS_LISTEN").unwrap_or_else(|_| "0.0.0.0:8080".into()).parse()?;
    let upstream = env::var("AEGIS_UPSTREAM").unwrap_or_else(|_| "http://127.0.0.1:8000".into());
    let policy_path = env::var("AEGIS_POLICY").unwrap_or_else(|_| "/var/lib/aegis/weights/active.bin".into());
    let policy = Arc::new(policy::Policy::open(&policy_path)?);
    let client = Client::builder(TokioExecutor::new()).build_http::<Incoming>();
    let st = Arc::new(State { upstream, client, policy });
    let lst = TcpListener::bind(listen).await?;
    tracing::info!(%listen, "aegis-proxy up");
    loop {
        let (sock, _) = lst.accept().await?;
        let st = st.clone();
        tokio::spawn(async move {
            let svc = service_fn(move |req| handle(req, st.clone()));
            if let Err(e) = hyper::server::conn::http1::Builder::new()
                .serve_connection(TokioIo::new(sock), svc)
                .await
            {
                tracing::debug!("conn closed: {e}");
            }
        });
    }
}
