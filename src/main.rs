pub mod odb;
pub mod repository;

use repository::Repository;

fn main() {
    let repo = Repository::init("./test/testinit_git").unwrap();
    println!("Repo: {:?}", repo);
}
