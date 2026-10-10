use std::io;
use std::process::ExitCode;

use rs_redb2tablenames::path2names2stdout;

fn io_envkey2val(key: &'static str) -> impl Fn() -> String {
    move || std::env::var(key).unwrap_or_default()
}

fn io_redb_filename() -> impl Fn() -> String {
    io_envkey2val("ENV_REDB_FILENAME")
}

fn io_main() -> impl FnOnce() -> Result<(), io::Error> {
    || {
        let p: String = io_redb_filename()();
        path2names2stdout(p)
    }
}

fn sub() -> Result<(), io::Error> {
    io_main()()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        let redb_filename: String = io_redb_filename()();
        eprintln!("ENV_REDB_FILENAME: {redb_filename}");
        ExitCode::FAILURE
    })
}
