use oracle::{Connection, Result, Version};

fn main() -> Result<()> {
    let client_ver = Version::client()?;
    println!("Oracle Client Version: {}", client_ver);

    let conn = Connection::connect("dbati", "dbati", "//localhost/XE")?;
    let (server_ver, banner) = conn.server_version()?;
    println!("Oracle Server Version: {}", server_ver);
    println!("--- Server Version Banner ---");
    println!("{}", banner);
    println!("-----------------------------");
    Ok(())
}