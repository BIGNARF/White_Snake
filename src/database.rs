use sqlite::{ Statement, Connection};
use std::{error::{Error}, str::FromStr};

pub struct AssetHolder{
    connection: Connection
}

impl AssetHolder{
    pub fn new()->Result<AssetHolder,Box<dyn Error>>{
        Ok(AssetHolder { 
            connection: sqlite::open("assets.db")? 
    })
    }
    
    pub fn Instantiate(&self,BlueprintID: i64)->Result<GameObject,Box<dyn Error>> {
        let query ="SELECT * FROM blueprints WHERE id = :blueprint;";
    
        let mut statement = self.connection.prepare(query)?;
        statement.bind((":blueprint",BlueprintID))?;
        
        while let Ok(sqlite::State::Row) = statement.next() {
            statement.read::<Vec<u8>, _>("file").unwrap();
        }
        Ok(GameObject{ID:0,name:"".to_string()})
    }   
}

pub struct GameObject{
    pub ID :u32,
    pub name :String,
    //pub parts :Vec<Component>
}

pub struct Component{
    pub Module :u32,
    pub ID :u32,
}

