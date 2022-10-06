# Reverse cached proxy

Reverse proxy made in Rust.
Accepts requests and forwards the original request to an origin server. For each request, keeps an in-memory cache of the response body with a TTL of 30 seconds. Use the cache data if the request is made multiple times.

## Main libraries used

- [Hyper]: A fast and correct HTTP implementation for Rust.
- [Tokio]: A runtime for writing reliable, asynchronous, and slim applications
- [Endorphin]: Key-Value based in-memory cache library which supports Custom Expiration Policies
- [Hyper-tls]: Provides an HTTPS connector for use with hyper.

## How to start

Clone this repo and run (for testing):
```sh
cargo run
```
For release:
```sh
cargo build --release
```

Set your proxy as `127.0.0.1:3000` if you test from a web browser.
Example `curl` command:
```sh
curl --proxy "127.0.0.1:3000" "http://www.google.com/"
```
## Metrics
Using the same target url:
```sh
ab -c 100 -n 10000 -X 127.0.0.1:3000 http://www.google.com/
```
```
Concurrency Level:      100
Time taken for tests:   18.315 seconds
Complete requests:      10000
Failed requests:        0
Total transferred:      151600000 bytes
HTML transferred:       150810000 bytes
Requests per second:    545.99 [#/sec] (mean)
Time per request:       183.154 [ms] (mean)
Time per request:       1.832 [ms] (mean, across all concurrent requests)
Transfer rate:          8083.17 [Kbytes/sec] received
```

## TO-DOs

- TLS support on the server side ([tsl-listener])
- Rate limiting for clients calling the proxy
- Read TTL, etc from configuration
- Rust tests for concurrent calls
- Refactor to implement requests with different protocols
- Metrics with multiple requests pointing to different domains

[hyper]: <https://github.com/hyperium/hyper>
[tokio]: <https://github.com/tokio-rs/tokio>
[endorphin]: <https://github.com/ArtBlnd/endorphin>
[hyper-tls]: <https://github.com/hyperium/hyper-tls>
[tsl-listener]: <https://github.com/tmccombs/tls-listener>
