package ipc

import (
	"bytes"
	"testing"
)

func TestRoundTrip(t *testing.T) {
	var b bytes.Buffer
	if err := WriteFrame(&b, []byte("hello")); err != nil {
		t.Fatal(err)
	}
	got, err := ReadFrame(&b)
	if err != nil || string(got) != "hello" {
		t.Fatalf("got %q err %v", got, err)
	}
}

func TestOversizeRejected(t *testing.T) {
	b := bytes.NewReader([]byte{0xFF, 0xFF, 0xFF, 0xFF})
	if _, err := ReadFrame(b); err == nil {
		t.Fatal("expected error")
	}
}
