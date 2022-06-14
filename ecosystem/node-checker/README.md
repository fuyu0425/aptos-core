While relying on my branch of poem, if you ever update that, run this from the repo root to update the dep locally:
```
cargo update -p poem -p poem-openapi
```

## Confirming that the preconfiguration of target node works
These should be turned into actual integration tests.

```
$ cargo run -- -d --baseline-node-url 'http://fullnode.devnet.aptoslabs.com/' --target-node-url http://localhost --allow-preconfigured-test-node-only
$ curl -s -I localhost:20121/api/check_node | head -n 1
HTTP/1.1 405 Method Not Allowed
$ curl -s -I localhost:20121/api/check_preconfigured_node | head -n 1
HTTP/1.1 200 OK
```

```
$ cargo run -- -d --baseline-node-url 'http://fullnode.devnet.aptoslabs.com/'
$ curl -s -I localhost:20121/api/check_node | head -n 1
HTTP/1.1 200 OK
$ curl -s -I localhost:20121/api/check_preconfigured_node | head -n 1
HTTP/1.1 405 Method Not Allowed
```
