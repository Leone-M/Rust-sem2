extern crate threads_pool;
use std::time::Instant;

fn main() {
    let start  = Instant::now();
    let pool = threads_pool::ThreadPool::new(2);
    for _ in 0..100000 {
        match pool.execute(|| {
            let _: i32 = 1+1;
        }) {
            Ok(_) => {}
            Err(_) => {println!("error occured!")}
        }
    }

    let duration = start.elapsed();
    println!("Time elapsed: {duration:#?}")
}
