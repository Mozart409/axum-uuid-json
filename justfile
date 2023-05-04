alias b := build
alias r := run

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

run: clear build
	docker run -p 3000:3000 mozart409/axum-uuid-json:latest 