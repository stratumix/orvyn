build:
    cargo build
escalate-and-run:
    run0 ./target/debug/orvyn
run: build && escalate-and-run
