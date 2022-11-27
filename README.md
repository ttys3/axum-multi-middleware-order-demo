# axum multiple middleware executing order

## Router::layer order

https://docs.rs/axum/latest/axum/middleware/index.html#ordering

When you add middleware with `Router::layer` (or similar) 
all previously added routes will be **wrapped** in the middleware.  
Generally speaking, this results in middleware being executed **from bottom to top**.

```rust
    let app = Router::new().route("/", get(handler))
    .layer(middleware::from_fn(my_middleware1))
    .layer(middleware::from_fn(my_middleware2))
    .layer(middleware::from_fn(my_middleware3));
```

```
         requests
            |
            v
+------ middleware3 ------+
| +---- middleware2 ----+ |
| | +-- middleware1 --+ | |
| | |               | | | |
| | |    handler    | | | |
| | |               | | | |
| | +-- middleware1 --+ | |
| +---- middleware2 ----+ |
+------ middleware3 ------+
            |
            v
        responses
```

output log:
```
hook before run: "middleware3"
hook before run: "middleware2"
hook before run: "middleware1"
exec the handler
hook after run: "middleware1"
hook after run: "middleware2"
hook after run: "middleware3"
```

## tower::ServiceBuilder order

https://docs.rs/tower/0.4.13/tower/builder/struct.ServiceBuilder.html#order

its recommended to use tower::ServiceBuilder to apply multiple middleware at once,
instead of calling layer (or route_layer) repeatedly

but when use tower::ServiceBuilder to apply multiple middleware at once, 
the order is reversed compared to `Router::layer`.

Layers that are added first will be called with the request first.

```rust
    let app = Router::new().route("/", get(handler))
        .layer(
    ServiceBuilder::new()
        .layer(middleware::from_fn(my_middleware1))
        .layer(middleware::from_fn(my_middleware2))
        .layer(middleware::from_fn(my_middleware3)));
```

```
         requests
            |
            v
+------ middleware1 ------+
| +---- middleware2 ----+ |
| | +-- middleware3 --+ | |
| | |               | | | |
| | |    handler    | | | |
| | |               | | | |
| | +-- middleware3 --+ | |
| +---- middleware2 ----+ |
+------ middleware1 ------+
            |
            v
         responses
```

output log:
```
before next.run: "middleware1"
before next.run: "middleware2"
before next.run: "middleware3"
exec the handler
after next.run: "middleware3"
after next.run: "middleware2"
after next.run: "middleware1"
```