// Week 1: Unix-socket listener. Accepts the Rust proxy's connection and logs every
// SecurityAuditEvent. Reload commands, signature checks and ClickHouse arrive in Week 4.
package main

import (
	"errors"
	"io"
	"log"
	"net"
	"os"

	ipcpb "aegisstream/gen/ipc"
	"aegisstream/internal/ipc"

	"google.golang.org/protobuf/proto"
)

func serve(c net.Conn) {
	defer c.Close()
	for {
		raw, err := ipc.ReadFrame(c)
		if err != nil {
			if !errors.Is(err, io.EOF) {
				log.Printf("read: %v", err)
			}
			return
		}
		var ev ipcpb.SecurityAuditEvent
		if err := proto.Unmarshal(raw, &ev); err != nil {
			log.Printf("bad frame: %v", err)
			continue
		}
		log.Printf("audit req=%s tenant=%s action=%s rule=%d score=%.3f ns=%d",
			ev.RequestId, ev.TenantId, ev.ActionTaken, ev.TriggeredRuleId, ev.ThreatScore, ev.InspectionTimeNs)
	}
}

func main() {
	path := os.Getenv("AEGIS_SOCK")
	if path == "" {
		path = "/var/run/aegis.sock"
	}
	_ = os.Remove(path)
	l, err := net.Listen("unix", path)
	if err != nil {
		log.Fatal(err)
	}
	defer l.Close()
	log.Printf("aegis-node-agent listening on %s", path)
	for {
		c, err := l.Accept()
		if err != nil {
			log.Fatal(err)
		}
		go serve(c)
	}
}
