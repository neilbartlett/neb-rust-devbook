Simple web service with logging.

This uses the actix_web framework to implement a simple webservice.

To use the service,

```
cargo run --bin actix_web_simple
```

from the top level and the use

http://127.0.0.1/hello/name

This should return a "hello name!" page.