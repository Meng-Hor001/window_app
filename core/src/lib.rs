pub mod model;

use model::{Column, DatabaseSchema, Table};

pub fn connection_message() -> String {
    "Hello Hor! React → Tauri → Rust Core works!".to_string()
}

pub fn sample_schema() -> DatabaseSchema {
    DatabaseSchema {
        name: "demo_db".to_string(),
        tables: vec![
            Table {
                name: "users".to_string(),
                columns: vec![
                    Column {
                        name: "id".to_string(),
                        data_type: "BIGINT".to_string(),
                        nullable: false,
                        primary_key: true,
                    },
                    Column {
                        name: "username".to_string(),
                        data_type: "VARCHAR(100)".to_string(),
                        nullable: false,
                        primary_key: false,
                    },
                    Column {
                        name: "email".to_string(),
                        data_type: "VARCHAR(255)".to_string(),
                        nullable: true,
                        primary_key: false,
                    },
                ],
            },
        ],
    }
}
