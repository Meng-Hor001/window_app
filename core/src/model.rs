use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Column {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub primary_key: bool,
}

#[derive(Debug, Serialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, Serialize)]
pub struct DatabaseSchema {
    pub name: String,
    pub tables: Vec<Table>,
}
