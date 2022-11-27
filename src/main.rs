use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{middleware, routing::get, Router};
use std::net::SocketAddr;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    // let app = Router::new()
    //     .route("/", get(handler))
    //     .layer(middleware::from_fn(my_middleware1))
    //     .layer(middleware::from_fn(my_middleware2))
    //     .layer(middleware::from_fn(my_middleware3));

    // https://docs.rs/axum/latest/axum/middleware/index.html#ordering
    // When you add middleware with Router::layer (or similar) all previously added routes will be wrapped in the middleware.
    // Generally speaking, this results in middleware being executed from bottom to top.
    /*
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
         */
    // output log:
    // before next.run: "middleware3"
    // before next.run: "middleware2"
    // before next.run: "middleware1"
    // exec the handler
    // after next.run: "middleware1"
    // after next.run: "middleware2"
    // after next.run: "middleware3"

    // its recommended to use tower::ServiceBuilder to apply multiple middleware at once,
    // instead of calling layer (or route_layer) repeatedly
    let app = Router::new().route("/", get(handler)).layer(
        ServiceBuilder::new()
            .layer(middleware::from_fn(my_middleware1))
            .layer(middleware::from_fn(my_middleware2))
            .layer(middleware::from_fn(my_middleware3)),
    );

    // but when use tower::ServiceBuilder to apply multiple middleware at once, the order is reversed to `Router::layer`
    // https://docs.rs/tower/0.4.13/tower/builder/struct.ServiceBuilder.html#order
    // Layers that are added first will be called with the request first.
    /*
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
         */
    // output log:
    // before next.run: "middleware1"
    // before next.run: "middleware2"
    // before next.run: "middleware3"
    // exec the handler
    // after next.run: "middleware3"
    // after next.run: "middleware2"
    // after next.run: "middleware1"

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    println!("exec the handler");
    "Hello, World!"
}

async fn my_middleware1<B>(req: Request<B>, next: Next<B>) -> Response {
    println!("before next.run: {:?}", "middleware1");
    let rsp = next.run(req).await;
    println!("after next.run: {:?}", "middleware1");
    rsp
}

async fn my_middleware2<B>(req: Request<B>, next: Next<B>) -> Response {
    println!("before next.run: {:?}", "middleware2");
    let rsp = next.run(req).await;
    println!("after next.run: {:?}", "middleware2");
    rsp
}

async fn my_middleware3<B>(req: Request<B>, next: Next<B>) -> Response {
    println!("before next.run: {:?}", "middleware3");
    let rsp = next.run(req).await;
    println!("after next.run: {:?}", "middleware3");
    rsp
}
