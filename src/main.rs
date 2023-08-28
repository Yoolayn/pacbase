use std::process::Command;
use rusqlite::{Connection, Result};

const DB_PATH: &str = "mydb.db";
const CACHE_PATH: &str = "/store/cache";

fn main() {
    let conn = match Connection::open(DB_PATH) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Failed to open db connection");
            return;
        }
    };
    match update(&conn) {
        Ok(_) => println!("done :)"),
        Err(_) => println!("error :(")
    };
}

fn update(conn: &Connection) -> Result<()> {
    // let conn = Connection::open(DB_PATH)?;
    let ls = Command::new("ls")
        .current_dir(CACHE_PATH)
        .output()
        .expect("Error running ls");

    conn.execute("CREATE TABLE IF NOT EXISTS Names (
            name_id INT PRIMARY KEY,
            package_name VARCHAR(255) NOT NULL
            );",
        [])?;
    conn.execute("CREATE TABLE IF NOT EXISTS Versions (
            version_id INT PRIMARY KEY,
            name_id INT,
            package_version VARCHAR(50) NOT NULL,
            FOREIGN KEY (name_id) REFERENCES Names(name_id)
            );",
        [])?;
    conn.execute("CREATE TABLE IF NOT EXISTS Files (
            file_id INT PRIMARY KEY,
            full_file_name VARCHAR(255) NOT NULL,
            version_id INT,
            FOREIGN KEY (version_id) REFERENCES Versions(version_id)
            );",
        [])?;

    if ls.status.success() {
        if let Ok(stdout) = String::from_utf8(ls.stdout) {
            let packages: Vec<&str> = stdout
                .split('\n')
                .collect();
            for mut package in packages.iter() {
                println!("package: {}", package);
            }
        }
    }

    Ok(())
}
