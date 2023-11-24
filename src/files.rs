use std::{path::PathBuf, fs};


pub struct PythonFile {
    pub content: String,
    pub filename: String
}

fn get_files_in_a_directory(dir: PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    
    for entry in dir.read_dir().unwrap() {
        files.push(entry.unwrap().path())
    }

    files
}

pub fn read_file(file: &PathBuf) -> PythonFile {
    let filename = file.as_os_str().to_str().expect("Invalid filename.").to_string();
    let content: String = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(_) => panic!("Unable to read file.")
    };

    PythonFile { content, filename }
}

pub fn get_files_list(provided: Vec<PathBuf>, recursive: bool) -> Vec<PathBuf> {
    provided.into_iter().map(|p| {
        if p.exists() == false {
            let filename = p.file_name().unwrap().to_str().unwrap();
            panic!("File {} does not exists.", filename)
        } else if recursive && p.is_dir() {
            get_files_list(get_files_in_a_directory(p), recursive)
        } else if p.is_file() {
            vec![p]
        } else {
            [].to_vec()
        }
    }).flatten().collect()
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn assert_files_list() {
        let base_dir = PathBuf::from("./python-examples");
        let output = get_files_list(vec![base_dir], true);
        let filenames: Vec<&str> = output.iter().map(|p| p.as_os_str().to_str().unwrap()).collect();

        assert_eq!(filenames, vec![
            "./python-examples/test_sample.py",
            "./python-examples/folder/test_1.py",
            "./python-examples/test_sample_complex.py",
        ])
    }
}