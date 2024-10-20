use std::sync::Arc;

use async_executor::{Executor, with_thread_pool};

async fn async_main(_ex: &Executor<'_>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Hello, world!");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // create executor
    let ex = Arc::new(Executor::new());

    // run executor on thread pool
    with_thread_pool(&ex, || async_io::block_on(async_main(&ex)))
}
