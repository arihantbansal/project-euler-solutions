pub fn run() {
    let mut line = String::new();
    println!("Enter :");
    std::io::stdin().read_line(&mut line)?;
    println!("Hello , {}", line);
}