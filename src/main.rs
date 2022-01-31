pub mod object;
pub mod repository;

use std::convert::AsRef;

fn main() {
    let repo = repository::Repository::find_parent(".").unwrap().unwrap();
    println!("Repo: {:?}", repo);

    let obj = repo
        .object_read(&"4b825dc642cb6eb9a060e54bf8d69288fbee4904")
        .unwrap()
        .unwrap();
    println!("Empty tree: {:?}", obj);
}
