use super::object::{ObjectType};
use super::repository::Repository;
use std::error::Error;
use std::env;
use std::io::{stdout, Write};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "cat-file")]
pub struct CatFileCommand {
    /// Git object type
    #[clap(parse(try_from_str = ObjectType::try_from))]
    r#type: ObjectType,

    /// Git object ID
    id: String,
}

impl super::Command for CatFileCommand {
    fn exec(&self) -> Result<u8, Box<dyn Error>> {
        let cwd = env::current_dir()?;
        if let Some(repo) = Repository::find_parent(cwd)? {
            let obj_data = cat_file(&repo, &self.r#type, &self.id)?;
            stdout().write_all(&obj_data)?;
        } else {
            eprintln!("Not in a Git Repository");
            return Ok(1);
        }
        Ok(0)
    }
}

pub fn cat_file<'a>(
    repo: &'a Repository,
    obj_type: &ObjectType,
    obj_id: &impl AsRef<str>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let obj_id = repo.object_resolve(obj_id, None, Some(false));
    match repo.object_read(obj_id) {
        Ok(opt) => match opt {
            Some(obj) => Ok(obj.into()),
            None => Err("No such object in repository".into()),
        },
        Err(e) => return Err(format!("Failed to read object: {:?}", e).into())
    }
}

