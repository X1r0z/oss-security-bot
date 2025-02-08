use rusqlite::{fallible_streaming_iterator::FallibleStreamingIterator, Connection};

pub fn create_connection(db_name: &str) -> anyhow::Result<Connection> {
    let conn = Connection::open(db_name)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS mailing_list (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mail TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn select_mail(conn: &Connection, subject: &str) -> anyhow::Result<bool> {
    let mut stmt = conn.prepare("SELECT * FROM mailing_list WHERE mail = ?")?;
    let rows = stmt.query([subject])?;

    Ok(rows.count()? > 0)
}

pub fn insert_mail(conn: &Connection, mail: &str) -> anyhow::Result<()> {
    conn.execute("INSERT INTO mailing_list (mail) VALUES (?)", [mail])?;
    Ok(())
}
