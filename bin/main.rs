use rust_crate_example;

fn main() {
    let config = rust_crate_example::GlobalConfig::read_config();
    println!("{:#?}", config);
}