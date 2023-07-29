#!/bin/bash
set -e

rustup uninstall 1.70.0

go install golang.org/x/tools/cmd/goimports@v0.9.3
go install github.com/golangci/golangci-lint/cmd/golangci-lint@v1.53.3

sudo apt-get update && sudo apt-get install -y dotnet-sdk-7.0
