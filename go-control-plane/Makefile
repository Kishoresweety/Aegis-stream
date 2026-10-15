# Requires protoc + protoc-gen-go:  go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
proto:
	protoc -I ../contracts --go_out=gen/ipc --go_opt=paths=source_relative ../contracts/aegis_ipc.proto
build: proto
	go build ./...
test: proto
	go test ./...
