#[cfg(not(feature = "web"))]
fn main() {
    println!("Hello World in Native");
}


#[cfg(feature = "web")]
fn main() {
}