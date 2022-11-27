use oracle::{Connection, Result, Version};

fn main() -> Result<()> {
    let username = "dbati";
    let password = "dbati";
    let database = "//localhost/XE";
    let sql = "select * from dbati.temp";

    let conn = Connection::connect(username, password, database)?;
    let mut stmt = conn.statement(sql).build()?;
    let rows = stmt.query(&[])?;

    // print column types
    // for (idx, info) in rows.column_info().iter().enumerate() {
    //     if idx != 0 {
    //         print!(",");
    //     }
    //     print!("{}", info);
    // }
    // println!();

    for row_result in rows {
        // print column values
        for (idx, val) in row_result?.sql_values().iter().enumerate() {
            if idx != 0 {
                print!(",");
            }
            print!("{}", val);
        }
        println!();
    }
    Ok(())
}