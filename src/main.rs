fn main() {
    usage();
}

fn usage() {
    let version = get_version();
    println!("tinymd, a markdown compiler written by Jonir227");
    println!("Version: {}", version);
}

fn get_version() -> u16 {
    1
}
