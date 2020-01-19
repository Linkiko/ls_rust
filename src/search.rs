use std::{fs, io, path::PathBuf};

pub fn search(path: &str) -> Result<Vec<PathBuf>, io::Error> {
    let path_information: PathBuf = PathBuf::from(path);
    if path_information.is_file() {
        Ok(vec![path_information])
    } else {
        let entries = fs::read_dir(path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>();
        entries
    }
}
