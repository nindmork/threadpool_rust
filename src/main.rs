use std::f64; 
mod mythreadpool;
use mythreadpool::ThreadPool;


struct Data {
    a: f64,
    b: f64,
    
}

fn calcu_late(param: &Data) {
    
    let product = param.a * param.b;
    let logarithm = f64::ln(product.into()); 
    let square_root = f64::sqrt(product.into()); 
    let hypotenuse = f64::hypot(param.a.into(), param.b.into());
    println!("a: {} b: {}", param.a, param.b);
    println!("a*b: {}", product);
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
    let pool = ThreadPool::new(2);

    // Submit the work to the pool
    pool.execute(move || {
        calcu_late(&work1);
    });
    pool.execute(move || {
        calcu_late(&work2);
    });
    pool.execute(move || {
        calcu_late(&work3);
    });
    pool.execute(move || {
        calcu_late(&work4);
    });

}




