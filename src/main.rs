#![feature(stmt_expr_attributes, proc_macro_hygiene)]
use futures::stream::Stream;
use futures_async_stream::for_await;

async fn collect(stream: impl Stream<Item = i32>) -> Vec<i32> {
    let mut vec = Vec::new();
    #[for_await]
    for value in stream.foo() {
        vec.push(value);
    }
    vec
}

fn main() {
    println!("Hello, world!");
}
