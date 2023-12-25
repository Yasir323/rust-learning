use std::error::Error;

fn main() {
    print!("Result: {}", threaded_version().unwrap());
}

fn sync_version() {
    let mut _x: u128 = 0u128;
    for i in 1..500_000_000 {
        _x += i;
    }
}

fn threaded_version() -> std::thread::Result<u128>{
    let handle = std::thread::spawn(|| {
        let mut x: u128 = 0;
        for i in 1..500_000_000 {
            x += i;
        }
        x
    });
    let result = handle.join()?;
    Ok(result)
    // OR
    // Ok(handle.join()?)
}
