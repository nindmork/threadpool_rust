use std::f64; 
mod mythreadpool;
use mythreadpool::ThreadPool;


struct Data {
    a: f64,
    b: f64,
    
}

fn add(param: &Data) {
    
    let product = param.a * param.b;
    let logarithm = f64::ln(product.into()); 
    let square_root = f64::sqrt(product.into()); 
    let hypotenuse = f64::hypot(param.a.into(), param.b.into());
    println!("Product: {}", product);
    println!("Logarithm: {}", logarithm);
    println!("Square Root: {}", square_root);
    println!("Hypotenuse: {}", hypotenuse);

}

fn main() {
    // Create some work to do
    let work1 = Data { a: 4.0, b: 2.0 };
    let work2 = Data { a: 5.0, b: 9.0 };
    let work3 = Data { a: 6.0, b: 8.0 };
    let work4 = Data { a: 20.0, b: 9.0 };
    // Initialize the thread pool
    let pool = ThreadPool::new(3);

    // Submit the work to the pool

    pool.execute(move || {
        add(&work1);
    });
    pool.execute(move || {
        add(&work2);
    });
    pool.execute(move || {
        add(&work3);
    });
    pool.execute(move || {
        add(&work4);
    });
    // Sleep for a while to allow the threads to finish (optional)
    //thread::sleep(std::time::Duration::from_secs(3));
}
