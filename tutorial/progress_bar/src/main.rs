use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn loading_bar() {
    // 0 - black background,
    // A - Green Foreground
    print!("\x1b[0;32m");
    
    // Initialize chars for printing loading bar
    let a = 177 as char;
    let b = 219 as char;

    print!("\n\n\n\n");
    println!("\n\n\n\n\t\t\t\t\tLoading...\n\n");

    // Set the cursor to the starting
    // point of the loading bar
    print!("\t\t\t\t\t");

    // Print initial loading bar
    for _ in 0..26 {
        print!("{}", a);
        io::stdout().flush().unwrap(); // Flush the output immediately
    }
    print!("\r\t\t\t\t\t");

    // Print loading bar progress
    for _ in 0..26 {
        print!("{}", b);
        io::stdout().flush().unwrap(); // Flush the output immediately
        // Sleep for 1 second
        thread::sleep(Duration::from_secs(1));
    }

    // Reset color to default
    print!("\x1b[0m");
}

// Driver Code
fn main() {
    // Function Call
    loading_bar();
    println!(); // Move to the next line after the progress bar completes
}

