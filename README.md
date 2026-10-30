# castle-eval

Test target for [Castle](https://github.com/tzdanows/castle): a small axum service with
unit tests, a `.castle.toml`, CI, a Trivy filesystem scan, and one dependency pinned to a
version with a published advisory so Dependabot has something to report.

Public on purpose: code scanning is only available on public repositories for this
account. Nothing here is real.

```
cargo test
cargo run   # 127.0.0.1:3000: GET /healthz, GET /greet/{name} -> "Hello, <name>!", GET /slug/{text}, POST /sum {"values":[1,2]}
```
