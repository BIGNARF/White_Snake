use jammdb::{Data, Error, KVPair, DB,Tx};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct test_obj{
    pub name: String,
    pub bin: Vec<u8>
}


pub struct db_handler {
    assets: DB,
    //tx: Tx
}

impl db_handler  {
    pub fn new() -> Result<db_handler, Error> {
        Ok(db_handler {
            assets: DB::open("jammer.db")?
        })
    }

    pub fn add<T>(&self, key: &str,obj: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        let tx = self.assets.tx(true)?;


        let users_bucket = tx.create_bucket("users")?;

        let user_bytes = rmp_serde::to_vec(&obj).unwrap();

        users_bucket.put(key, user_bytes)?;

        tx.commit()?;
        Ok(())
    }
     
    pub fn read(&self,key: &str) -> Result<Vec<u8>, Error>
    {
        // open a read-only transaction to get the data
        let tx = self.assets.tx(false)?;
    
        // get the bucket we created in the last transaction
        let users_bucket = tx.get_bucket("users")?;
    
        // get the key / value pair we inserted into the bucket
        let data=users_bucket.get(key).unwrap();
        if data.is_kv() {
            let dbref=data.kv().value().to_vec();
            Ok(dbref)
        }
        else {
            Err(Error::KeyValueMissing)
        }
    } 
    
}



