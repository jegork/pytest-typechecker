use std::{fs, path::PathBuf};

use python_file::PythonFile;
pub mod parsed_python_file;
pub mod python_file;

fn get_files_in_a_directory(dir: PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in dir.read_dir().unwrap() {
        files.push(entry.unwrap().path())
    }

    files
}

pub fn read_file(file: &PathBuf) -> PythonFile {
    let filename = file
        .as_os_str()
        .to_str()
        .expect("Invalid filename.")
        .to_string();
    let content: String = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(_) => panic!("Unable to read file."),
    };

    PythonFile { content, filename }
}

pub fn get_files_list(provided: Vec<PathBuf>, recursive: bool) -> Vec<PathBuf> {
    provided
        .into_iter()
        .flat_map(|p| {
            if !p.exists() {
                let filename = p.file_name().unwrap().to_str().unwrap();
                panic!("File {} does not exists.", filename)
            } else if recursive && p.is_dir() {
                get_files_list(get_files_in_a_directory(p), recursive)
            } else if p.is_file() {
                vec![p]
            } else {
                [].to_vec()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use std::{collections::HashSet, fs::File, path::Path};

    use tempfile::{tempdir, TempDir};

    use super::*;

    fn generate_test_directory() -> anyhow::Result<TempDir> {
        let temp_dir = tempdir()?;
        File::create(temp_dir.path().join("python_file1.py"))?;
        File::create(temp_dir.path().join("python_file2.py"))?;

        fs::create_dir(temp_dir.path().join("subfolder"))?;
        File::create(temp_dir.path().join("subfolder").join("python_file3.py"))?;

        Ok(temp_dir)
    }

    fn get_str_from_path(pathbuf: &Path) -> String {
        pathbuf.to_str().unwrap().to_owned()
    }

    #[test]
    fn assert_files_list() -> anyhow::Result<()> {
        let base_dir: PathBuf = generate_test_directory()?.into_path();

        let output: Vec<PathBuf> = get_files_list(vec![base_dir.clone()], true);
        let filenames: HashSet<String> = output.iter().map(|p| get_str_from_path(p)).collect();

        let expected_filenames: HashSet<String> = vec![
            &base_dir.join("python_file1.py"),
            &base_dir.join("python_file2.py"),
            &base_dir.join("subfolder").join("python_file3.py"),
        ]
        .iter()
        .map(|p| get_str_from_path(p))
        .collect();

        assert_eq!(filenames, expected_filenames);

        Ok(())
    }
}
