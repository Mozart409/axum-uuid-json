alias b := build

default:
  just --list

clear:
	clear

build: clear
	docker build -t mozart409/axum-uuid-json:latest .

lint: clear
	cargo clippy

fix: clear
	cargo clippy --fix
 