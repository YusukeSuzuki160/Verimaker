use crate::get_module_list::get_module;
use std::path::{Path, PathBuf};
use std::io;
use walkdir::{WalkDir, DirEntry};
use std::collections::HashMap;

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

pub fn gen_file_module_list(path: PathBuf) -> io::Result<(HashMap<String, Vec<String>>, HashMap<String, Vec<String>>)> { // Get all modules from file under path
    let mut module_list = HashMap::new();
    let mut dependense_graph = HashMap::new();
    for file in files(&path) {
        let file_name = file.to_str().unwrap();
        match file_name.split(".").last() {
            Some("sv") => {},
            _ => continue,
        }
        let file_path = Path::new(file_name);
        let (result, dependensies) = get_module(file_path).unwrap();
        for module in result {
            if module_list.contains_key(file_name) {
                let module_list_vec : &mut Vec<String> = module_list.get_mut(file_name).unwrap();
                module_list_vec.push(module);
            } else {
                module_list.insert(file_name.to_string(), vec![module]);
            }
        }
        dependense_graph.extend(dependensies)
    }
    Ok((module_list, dependense_graph))
}


#[test]
fn test_apply_to_each_file() {
    let path = Path::new("./testcase/").to_path_buf();
    let (result, dependensies) = gen_file_module_list(path).unwrap();
    assert_eq!(result, {
        let mut map = HashMap::new();
        map.insert("./testcase/test1.sv".to_string(), vec!["reorder".to_string()]);
        map.insert("./testcase/test2.sv".to_string(), vec!["top".to_string()]);
        map
    });
    assert_eq!(dependensies, {
        let mut map = HashMap::new();
        map.insert("top".to_string(), vec!["FFT_unit".to_string(), "reorder".to_string()]);
        map
    });
}






