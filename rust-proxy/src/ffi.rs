//! Hand-written binding for contracts/mojoguard.h.
use std::time::Instant;

pub const VERDICT_PASS: i32 = 0;
#[allow(dead_code)]
pub const VERDICT_REDACTED: i32 = 1;
#[allow(dead_code)]
pub const VERDICT_BLOCK: i32 = 2;
#[allow(dead_code)]
pub const VERDICT_ERROR: i32 = -1;

extern "C" {
    fn mojo_inspect_window(
        window_ptr: *mut u8,
        window_len: usize,
        inspect_offset: usize,
        policy_mmap_ptr: *const u8,
        out_threat_score: *mut f32,
        out_rule_id: *mut u32,
    ) -> i32;
}

pub struct Inspection {
    pub verdict: i32,
    pub score: f32,
    pub rule_id: u32,
    pub elapsed_ns: u64,
}

/// `policy` must be a validated mapping (see policy.rs).
pub fn inspect(window: &mut [u8], inspect_offset: usize, policy: &[u8]) -> Inspection {
    let mut score = 0f32;
    let mut rule = 0u32;
    let t = Instant::now();
    // SAFETY: pointers come from live slices, lengths match, out-params are valid locals.
    let verdict = unsafe {
        mojo_inspect_window(
            window.as_mut_ptr(),
            window.len(),
            inspect_offset.min(window.len()),
            policy.as_ptr(),
            &mut score,
            &mut rule,
        )
    };
    Inspection { verdict, score, rule_id: rule, elapsed_ns: t.elapsed().as_nanos() as u64 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::tests::dummy_policy_bytes;

    // Friday integration target: cargo test calling mojo_inspect_window on a static array.
    #[test]
    fn pass_on_static_window() {
        let policy = dummy_policy_bytes();
        let mut w = *b"hello 4111-11";
        let r = inspect(&mut w, 0, &policy);
        assert_eq!(r.verdict, VERDICT_PASS);
        assert_eq!(&w, b"hello 4111-11");
    }

    #[test]
    fn error_on_bad_magic() {
        let mut policy = dummy_policy_bytes();
        policy[0] ^= 0xFF;
        let mut w = *b"x";
        assert_eq!(inspect(&mut w, 0, &policy).verdict, VERDICT_ERROR);
    }
}
