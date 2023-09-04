mod hello;
use hello::hello_filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = hello_filter();

    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;
}