use std::io;
use std::path::Path;

use redb::ReadOnlyDatabase;
use redb::ReadableDatabase;
use redb::TableHandle;

pub fn path2db<P>(p: P) -> Result<ReadOnlyDatabase, io::Error>
where
    P: AsRef<Path>,
{
    ReadOnlyDatabase::open(&p)
        .map_err(|e| format!("unable to open db at path {:#?}: {e}", p.as_ref()))
        .map_err(io::Error::other)
}

pub fn tablename_sink2stdout(name: &str) -> Result<(), io::Error> {
    println!("{name}");
    Ok(())
}

pub struct Db<R>(pub R);

impl<R> Db<R>
where
    R: ReadableDatabase,
{
    pub fn names2sink<S>(&self, sink: &mut S) -> Result<(), io::Error>
    where
        S: FnMut(&str) -> Result<(), io::Error>,
    {
        let tx = self.0.begin_read().map_err(io::Error::other)?;
        let handles = tx.list_tables().map_err(io::Error::other)?;
        for hndl in handles {
            let name: &str = hndl.name();
            sink(name)?;
        }
        Ok(())
    }
}

impl<R> Db<R>
where
    R: ReadableDatabase,
{
    pub fn names2sink_default(&self) -> Result<(), io::Error> {
        self.names2sink(&mut tablename_sink2stdout)
    }
}

pub fn path2names2stdout<P>(p: P) -> Result<(), io::Error>
where
    P: AsRef<Path>,
{
    let db: ReadOnlyDatabase = path2db(p)?;
    Db(db).names2sink_default()
}
