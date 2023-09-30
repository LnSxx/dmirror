
fn main() {
    let dir = std::env::current_dir().expect("Problem getting working directory");

    let dir_name = dir
        .into_os_string()
        .into_string()
        .expect("Problem reading working directory");

    println!("Current working directory");
    println!("{}", dir_name);
}
