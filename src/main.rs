fn main() {
    usage();
}

fn usage() {
    println!("tinymd, a markdown compiler written by Jonir227");
    println!("Version: {}", get_version());
}

fn get_version() -> u16 {
    1
}
