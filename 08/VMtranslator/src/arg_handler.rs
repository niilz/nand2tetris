use std::path::{ Path, PathBuf };

pub fn path_builder(path: &Path) -> (String, PathBuf, Vec<PathBuf>) {
    
    let file_stem;

    let output_file = (match path.is_dir() {
            true  => {
                let file = path.file_name().unwrap();
                file_stem = file.to_str().unwrap();
                path.join(file)
            },
            false => {
                file_stem = path.file_stem().unwrap().to_str().unwrap();
                path.to_path_buf()
            },
        }).with_extension("asm");

    let file_paths: Vec<PathBuf> = match path.is_dir() {
        true => path.read_dir()
                    .expect("Could not read dir")
                    .map(|path| path.unwrap().path())
                    .filter(|path| path.extension().unwrap() == "vm")
                    .collect(),
        false => vec![path.to_path_buf()],
    };

    println!("FILE PATHS {:?}", file_paths);

    (file_stem.to_string(), output_file, file_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_file_path() {
        let input_file = Path::new("../my/great/path/with/singleFile.vm");
        let file_stem = "singleFile".to_string();
        let output_file = Path::new("../my/great/path/with/singleFile.asm").to_path_buf();
        let paths_to_vm_files = vec![Path::new("../my/great/path/with/singleFile.vm").to_path_buf()];

        assert_eq!(path_builder(input_file), (file_stem, output_file, paths_to_vm_files));
    }
    #[test]
    fn handle_dir_path() {
        let input_file = Path::new("../my/great/path/with/multiFiles/");
        let file_stem = "multiFiles".to_string();
        let output_file = Path::new("../my/great/path/with/multiFiles.asm").to_path_buf();
        let paths_to_vm_files = vec![Path::new("../my/great/path/with/multiFiles/").to_path_buf()];

        assert_eq!(path_builder(input_file), (file_stem, output_file, paths_to_vm_files));
    }
}