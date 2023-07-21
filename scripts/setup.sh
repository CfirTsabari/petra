#!/bin/bash
set -e

rustup uninstall 1.70.0
go install golang.org/x/tools/cmd/goimports@v0.9.3
go install github.com/golangci/golangci-lint/cmd/golangci-lint@v1.53.3
