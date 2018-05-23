extern crate tokio;
extern crate tokio_threadpool;
extern crate tokio_timer;

use tokio::prelude::*;

fn main() {
    let x = tokio_timer::sleep(std::time::Duration::from_secs(1000)).map_err(|_| ());
    let mut threadpool_builder = tokio_threadpool::Builder::new();
    threadpool_builder.pool_size(1);
    let mut threadpool = threadpool_builder.build();
    // let mut runtime = tokio::runtime::Builder::new()
    //     .threadpool_builder(threadpool_builder)
    //     .build()
    //     .unwrap();
    threadpool.spawn(x);
    threadpool.shutdown_on_idle().wait().unwrap();
}
