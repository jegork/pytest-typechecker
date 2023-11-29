use std::{fs, path::PathBuf};

use python_file::PythonFile;
pub mod parsed_python_file;
pub mod python_file;
use glob::glob;

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
        .flat_map(|v| {
            if v.is_dir() {
                let pattern = if recursive {
                    v.join("**").join("*.py")
                } else {
                    v.join("*.py")
                };

                let pattern = pattern.to_str().unwrap().to_string();

                let files = glob(&pattern).unwrap();
                let out: Vec<PathBuf> = files.filter_map(|f| f.ok()).collect();
                out
            } else {
                vec![v]
            }
        })
        .filter(|f| {
            if !f.exists() {
                panic!("File {} does not exist.", f.to_str().unwrap())
            }
            true
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

        fs::create_dir(temp_dir.path().join("subfolder").join("subsubfolder"))?;
        File::create(
            temp_dir
                .path()
                .join("subfolder")
                .join("subsubfolder")
                .join("python_file4.py"),
        )?;

        Ok(temp_dir)
    }

    fn get_str_from_path(pathbuf: &Path) -> String {
        pathbuf.to_str().unwrap().to_owned()
    }

    #[test]
    fn assert_files_list() -> anyhow::Result<()> {
        let base_dir: PathBuf = generate_test_directory()?.into_path();

        let output: Vec<PathBuf> = get_files_list(vec![base_dir.clone()], false);
        let filenames: HashSet<String> = output.iter().map(|p| get_str_from_path(p)).collect();

        let expected_filenames: HashSet<String> = vec![
            &base_dir.join("python_file1.py"),
            &base_dir.join("python_file2.py"),
        ]
        .iter()
        .map(|p| get_str_from_path(p))
        .collect();

        assert_eq!(filenames, expected_filenames);

        Ok(())
    }

    #[test]
    fn assert_files_list_recursive() -> anyhow::Result<()> {
        let base_dir: PathBuf = generate_test_directory()?.into_path();

        let output: Vec<PathBuf> = get_files_list(vec![base_dir.clone()], true);
        let filenames: HashSet<String> = output.iter().map(|p| get_str_from_path(p)).collect();

        let expected_filenames: HashSet<String> = vec![
            &base_dir.join("python_file1.py"),
            &base_dir.join("python_file2.py"),
            &base_dir.join("subfolder").join("python_file3.py"),
            &base_dir
                .join("subfolder")
                .join("subsubfolder")
                .join("python_file4.py"),
        ]
        .iter()
        .map(|p| get_str_from_path(p))
        .collect();

        assert_eq!(filenames, expected_filenames);

        Ok(())
    }
}
