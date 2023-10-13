use jammdb::{Data, Error, DB};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct User {
    username: String,
    password: String,
}

fn add_to_db() -> Result<(), Error> {
    let user = User {
        username: "my-user".to_string(),
        password: "my-password".to_string(),
    };

    // open a new database file and start a writable transaction
    let db = DB::open("my-database.db")?;
    let tx = db.tx(true)?;

    // create a bucket to store users
    let users_bucket = tx.create_bucket("users")?;

    // serialize struct to bytes and store in bucket
    let user_bytes = rmp_serde::to_vec(&user).unwrap();
    users_bucket.put("user1", user_bytes)?;

    // commit the changes so they are saved to disk
    tx.commit()?;

    Ok(())
}

fn read_from_db() -> Result<(), Error> {
    // open the existing database file
    let db = DB::open("my-database.db")?;

    // open a read-only transaction to get the data
    let tx = db.tx(false)?;

    // get the bucket we created in the last transaction
    let users_bucket = tx.get_bucket("users")?;

    // get the key / value pair we inserted into the bucket
    if let Some(data) = users_bucket.get("user1") {
        if data.is_kv() {
            let kv = data.kv();
            // deserialize into a user struct
            let db_user: User = rmp_serde::from_slice(kv.value()).unwrap();
            println!("pass:{}, name:{}", db_user.password, db_user.username);
            //assert_eq!(db_user, user);
        }
    }
    //Err(Error::BucketExists)
    Ok(())
}