use crossbeam_channel as channel;
use ignore::{DirEntry, WalkBuilder, WalkParallel, WalkState};
use std::thread;

type Walker = WalkParallel;
type Filterer = fn(&DirEntry) -> bool;
type Searcher = fn(WalkParallel, channel::Sender<DirEntry>);

pub fn get_directories(path: &str, ignore_filters_on: bool) -> Vec<String> {
    let walker = WalkBuilder::new(path)
        .standard_filters(ignore_filters_on)
        .build_parallel();
    run_worker_with_default_searcher(walker, directory_filterer)
}

pub fn get_files(path: &str, ignore_filters_on: bool) -> Vec<String> {
    let walker = WalkBuilder::new(path)
        .standard_filters(ignore_filters_on)
        .build_parallel();
    run_worker_with_default_searcher(walker, file_filterer)
}

pub fn get_files_and_directories(path: &str, ignore_filters_on: bool) -> Vec<String> {
    let walker = WalkBuilder::new(path)
        .standard_filters(ignore_filters_on)
        .build_parallel();
    run_worker_with_default_searcher(walker, file_and_directory_filterer)
}

pub fn run_worker_with_default_searcher(walker: Walker, pass_filter: Filterer) -> Vec<String> {
    let (tx, rx) = channel::unbounded::<DirEntry>();

    thread::spawn(move || {
        walker.run(|| {
            let tx = tx.clone();
            Box::new(move |result| {
                if let Ok(file) = result {
                    if pass_filter(&file) && tx.send(file).is_err() {
                        return WalkState::Quit;
                    }
                }
                WalkState::Continue
            })
        });
        drop(tx);
    });

    let mut results = Vec::new();
    for item in rx {
        results.push(item.path().to_str().unwrap().to_string());
    }

    results
}

pub fn run_worker(walker: Walker, searcher: Searcher) -> Vec<String> {
    let (tx, rx) = channel::unbounded::<DirEntry>();

    searcher(walker, tx);

    let mut results = Vec::new();
    for item in rx {
        results.push(item.path().to_str().unwrap().to_string());
    }

    results
}

fn directory_filterer(file: &DirEntry) -> bool {
    let filetype = file.file_type().unwrap();
    filetype.is_dir()
}

fn file_filterer(file: &DirEntry) -> bool {
    let filetype = file.file_type().unwrap();
    !filetype.is_dir()
}

fn file_and_directory_filterer(_file: &DirEntry) -> bool {
    true
}

#[cfg(test)]
mod tests {

    use crate::parallel::get_directories;
    use crate::parallel::get_files;
    use crate::parallel::get_files_and_directories;

    #[test]
    fn files_search() {
        let mut crate_dir = std::env::current_dir().unwrap();
        crate_dir.push("test");

        let files = get_files(crate_dir.to_str().unwrap(), false);
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

        let directories = get_directories(crate_dir.to_str().unwrap(), false);
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

        let files_and_directories = get_files_and_directories(crate_dir.to_str().unwrap(), false);
        assert_eq!(
            files_and_directories.len(),
            638,
            "Expected 638 files and directories, but found {} files and directories",
            files_and_directories.len()
        );
    }

}
