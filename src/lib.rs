#[no_mangle]
pub fn decode(path: String) -> String {
    println!("decoding path: {}", path);
    return String::from("Hello World!");
}

#[no_mangle]
pub fn add(a: usize, b: usize) -> usize {
    return a + b;
}
