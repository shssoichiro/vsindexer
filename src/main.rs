use std::{env::args, ffi::OsStr, process::Command};

use walkdir::WalkDir;

fn main() {
    let dirs = args().skip(1).collect::<Vec<_>>();

    if dirs.is_empty() {
        panic!("Usage: vsindexer [input_dirs]");
    }

    for dir in dirs {
        for file in WalkDir::new(dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().extension() == Some(OsStr::new("vpy")))
        {
            eprintln!("Indexing {}", file.path().to_string_lossy());
            let _ = Command::new("vspipe").arg("-i").arg(file.path()).status();
        }
    }

    eprintln!("Done");
}
