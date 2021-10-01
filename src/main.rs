mod lib;

fn main() {
    println!("Hello, world!");
    let msg = lib::decode("test.png".to_string());
    println!("msg: {}", msg);
}
