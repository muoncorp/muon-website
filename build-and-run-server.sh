#!/bin/sh

update_frontend () {
    (cd frontend-new && npm install && npm run build)
}

run_server () {
    killall target/release/muon-website-server
    cd server
    RUST_LOG="debug" nohup ~/.cargo/bin/cargo run --release >> /tmp/muon-website-server.log &
}

update_frontend && run_server
