use warp::Filter;

pub fn hello_filter() -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name))
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;

    #[tokio::test]
    async fn test_hello_endpoint() {
        // Create a test request
        let request = request().method("GET").path("/hello/warp");

        // Send it to the hello filter
        let hello = hello_filter();
        let resp = request.reply(&hello).await;

        // Assert that the response is as expected
        assert_eq!(resp.status(), warp::http::StatusCode::OK);
        assert_eq!(resp.body(), "Hello, warp!");
    }
}
