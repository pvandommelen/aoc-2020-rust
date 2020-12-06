use std::time;

pub fn time<T, F>(name: &str, work: F) -> T
where F: FnOnce() -> T
{
    let start = time::Instant::now();

    let result = work();
    
    let end = time::Instant::now();
    println!("Time '{}': {:?}", name, end - start);

    result
}

pub fn time_repeat<T, F>(name: &str, work: F, n: u32) -> T
where F: Fn() -> T
{
    let start = time::Instant::now();

    let mut result: Option<T> = None;
    for _ in 0..n {
        result = Some(work());
    }
    
    let end = time::Instant::now();
    println!("Time '{}': {:?}", name, (end - start) / n);

    result.expect("Expected work function to be executed atleast once")
}