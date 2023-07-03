use crate::get_module_list::get_module;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use walkdir::{WalkDir, DirEntry};

struct Files {
    inner: Box<dyn Iterator<Item = walkdir::Result<DirEntry>>>,
}

impl Iterator for Files {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.inner.next() {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_file() {
                        return Some(entry.into_path());
                    }
                }
                Err(_) => {},
            }
        }
        None
    }
}

fn files(path: &PathBuf) -> Files {
    Files { inner: Box::new(WalkDir::new(path).into_iter()) }
}

fn gen_file_module_list(path: PathBuf) -> io::Result<Vec<String>> {
    let mut module_list: Vec<String> = Vec::new();
    for file in files(&path) {
        let file_path = file.to_str().unwrap();
        let file_path = Path::new(file_path);
        let result = get_module(file_path);
        if let Ok(list) = result {
            for module in list {
                println!("module: {}", module);
                module_list.push(module);
            }
        }
    }
    Ok(module_list)
}


#[test]
fn test_apply_to_each_file() {
    let path = Path::new("./testcase/").to_path_buf();
    let result = gen_file_module_list(path).unwrap();
    assert_eq!(result, vec!["reorder".to_string(), "top".to_string()]);
}






