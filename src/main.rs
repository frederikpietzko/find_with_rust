use std::{
    fs, io,
    path::{Path, PathBuf},
};

use clap::Parser;

fn visit_dirs(dir: &Path, mut cb: impl FnMut(&fs::DirEntry) -> bool) -> io::Result<Vec<PathBuf>> {
    let mut hits = vec![];
    let mut queue = vec![dir.to_path_buf()];
    while !queue.is_empty() {
        let dir = queue.pop().unwrap();
        if dir.is_dir() {
            for e in fs::read_dir(dir)? {
                let entry = e?;
                if cb(&entry) {
                    hits.push(entry.path());
                }
                if entry.path().is_dir() {
                    queue.insert(0, entry.path());
                }
            }
        }
    }

    Ok(hits)
}

#[derive(Parser)]
struct Cli {
    name: String,
    #[clap(parse(from_os_str))]
    path: Option<PathBuf>,
}

fn main() {
    let args: Cli = Cli::parse();
    let path = args.path.or(Some(Path::new(".").to_path_buf())).unwrap();
    visit_dirs(&path, |entry| {
        entry.path().file_name().unwrap().eq(args.name.as_str())
    })
    .unwrap()
    .iter()
    .for_each(|e| println!("{:?}", e));
}
