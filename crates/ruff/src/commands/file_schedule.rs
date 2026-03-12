use std::cmp::Reverse;

#[derive(Debug)]
pub(crate) struct ScheduledFile<T> {
    pub(crate) file: T,
    pub(crate) size_hint: u64,
    pub(crate) cached: bool,
}

pub(crate) fn sort_non_cached_by_size<T>(files: Vec<ScheduledFile<T>>) -> Vec<T> {
    let (cached, mut non_cached): (Vec<_>, Vec<_>) =
        files.into_iter().partition(|file| file.cached);
    non_cached.sort_by_key(|file| Reverse(file.size_hint));

    cached
        .into_iter()
        .chain(non_cached)
        .map(|file| file.file)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{ScheduledFile, sort_non_cached_by_size};

    #[test]
    fn sorts_only_non_cached_files_by_size_descending() {
        let files = vec![
            ScheduledFile {
                file: "cached-1",
                size_hint: 1,
                cached: true,
            },
            ScheduledFile {
                file: "small",
                size_hint: 1,
                cached: false,
            },
            ScheduledFile {
                file: "cached-2",
                size_hint: 20,
                cached: true,
            },
            ScheduledFile {
                file: "large",
                size_hint: 10,
                cached: false,
            },
        ];

        assert_eq!(
            sort_non_cached_by_size(files),
            vec!["cached-1", "cached-2", "large", "small"]
        );
    }
}
