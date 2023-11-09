use std::path;

use sqlite::{ Statement, Connection};

pub struct AssetHolder{
    connection: Connection
}

impl AssetHolder{
    pub fn new<T>(path :T)->AssetHolder
    where T:AsRef<std::path::Path>
    {
        AssetHolder { connection: sqlite::open(path).unwrap() }
    }

    pub fn build(&self, Table: &str)->sqlite::Result<()>{
        let query =format!("CREATE TABLE {} (name TEXT,file BLOB);",Table);
        self.connection.execute(query.as_str())?;
        Ok(())
    }
    
    pub fn add_blob(&self,table: &str,name: &str, data: &[u8])->sqlite::Result<()> {
        let query = format!("INSERT INTO {} VALUES (:name,:data);", table);
        let mut statement= self.connection.prepare(query.as_str())?;
        statement.bind((":name",name));
        statement.bind((":data",data));
        while let Ok(sqlite::State::Row) = statement.next() {
            statement.read::<String, _>("name").unwrap();
        }
        Ok(())
    }
    
    pub fn read_blob(&self,Table: &str,Value: &str)->sqlite::Result<Vec<u8>> {
        let query =format!("SELECT * FROM {} WHERE name = :name;",Table);
    
        let mut statement = self.connection.prepare(query.as_str()).unwrap();
        statement.bind((":name",Value))?;
        
        let mut res :Vec<u8>=vec![];
        while let Ok(sqlite::State::Row) = statement.next() {
            res=statement.read::<Vec<u8>, _>("file").unwrap();
            statement.read::<Vec<u8>, _>("file").unwrap();
        }
        Ok(res)
    }
     
}