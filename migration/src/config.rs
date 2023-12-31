use std::{env::var, process::Command};

use sea_orm_migration::sea_orm::{Database, DatabaseConnection, DbErr};

pub struct DB {
    name: String,
    user: String,
    host: String,
    port: String,
    password: String,
}

impl DB {
    pub fn init() -> Self {
        Self {
            name: var("PGDATABASE").expect(fmt_err("PGDATABASE").as_str()),
            user: var("PGUSER").expect(fmt_err("PGUSER").as_str()),
            host: var("PGHOST").expect(fmt_err("PGHOST").as_str()),
            port: var("PGPORT").expect(fmt_err("PGPORT").as_str()),
            password: var("PGPASSWORD").expect(fmt_err("PGPASSWORD").as_str()),
        }
    }

    fn url(&self) -> String {
        let user = &self.user;
        let pswd = &self.password;
        let host = &self.host;
        let port = &self.port;
        let name = &self.name;
        format!("postgresql://{user}:{pswd}@{host}:{port}/{name}")
    }

    pub fn generate_entities(&self) -> &Self {
        println!("Generating database entities");
        let url = self.url();
        let out_dir = format!("entity/src/entities");
        let command =
            format!("/home/Vice/.cargo/bin/sea-orm-cli generate entity -u {url} -o {out_dir}");
        println!("{command}");

        Command::new(command).status().expect("Entity Generation");
        println!("Generated database entities");
        self
    }

    pub async fn connect(&self) -> Result<DatabaseConnection, DbErr> {
        let url = self.url();
        let connection = Database::connect(url).await;
        return connection;
    }
}

fn fmt_err(name: &str) -> String {
    format!("{name} env var should be set")
}
