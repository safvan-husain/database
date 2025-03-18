mod utils;

use crate::utils::{database::Database, index::Index};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), std::io::Error> {
    // Create a shared database instance
    let mut c_db = Database::new("test_multi.db")?;
    let mut i_db =Database::new("test_multi.db")?;

    c_db.clear_database()?;
    i_db.clear_database()?;

    let mut threads = vec![];

    for i in 0..5 {
        let c_db_clone = c_db.clone();
        let i_db_clone = Arc::clone(&i_db);

        let thread = thread::spawn(move || {
            for j in 0..5 {
                Index::new(format!("at {i} on {j}"),&mut c_db,&mut i_db).unwrap();
            }
        });
        threads.push(thread);
    }
    for handle in threads {
        handle.join().unwrap();
    }
    Ok(())
}

/*

* */