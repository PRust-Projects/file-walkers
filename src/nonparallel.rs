use walkdir::WalkDir;

pub fn get_directories(path: &str) -> Vec<String> {
    let mut directories = Vec::new();
    for entry in WalkDir::new(path)
        .contents_first(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            directories.push(entry.path().to_str().unwrap().to_string());
        }
    }

    directories
}

pub fn get_files(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path)
        .contents_first(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_dir() {
            files.push(entry.path().to_str().unwrap().to_string());
        }
    }

    files
}

pub fn get_files_and_directories(path: &str) -> Vec<String> {
    let mut files_and_directories = Vec::new();
    for entry in WalkDir::new(path)
        .contents_first(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        files_and_directories.push(entry.path().to_str().unwrap().to_string());
    }

    files_and_directories
}

#[cfg(test)]
mod tests {

    use crate::nonparallel::get_directories;
    use crate::nonparallel::get_files;
    use crate::nonparallel::get_files_and_directories;

    #[test]
    fn files_search() {
        let mut crate_dir = std::env::current_dir().unwrap();
        crate_dir.push("test");

        let files = get_files(crate_dir.to_str().unwrap());
        assert_eq!(
            files.len(),
            577,
            "Expected 577 files, but found {} files",
            files.len()
        );
    }

    #[test]
    fn directories_search() {
        let mut crate_dir = std::env::current_dir().unwrap();
        crate_dir.push("test");

        let directories = get_directories(crate_dir.to_str().unwrap());
        assert_eq!(
            directories.len(),
            61,
            "Expected 61 directories, but found {} directories",
            directories.len()
        );
    }

    #[test]
    fn files_and_directories_search() {
        let mut crate_dir = std::env::current_dir().unwrap();
        crate_dir.push("test");

        let files_and_directories = get_files_and_directories(crate_dir.to_str().unwrap());
        assert_eq!(
            files_and_directories.len(),
            638,
            "Expected 638 files and directories, but found {} files and directories",
            files_and_directories.len()
        );
    }

}
