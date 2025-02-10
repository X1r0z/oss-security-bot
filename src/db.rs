use rusqlite::{fallible_streaming_iterator::FallibleStreamingIterator, Connection};

pub fn create_connection(db_name: &str) -> anyhow::Result<Connection> {
    let conn = Connection::open(db_name)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS mailing_list (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mail_id TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn select_mail(conn: &Connection, mail_id: &str) -> anyhow::Result<bool> {
    let mut stmt = conn.prepare("SELECT * FROM mailing_list WHERE mail_id = ?")?;
    let rows = stmt.query([mail_id])?;

    Ok(rows.count()? > 0)
}

pub fn insert_mail(conn: &Connection, mail_id: &str) -> anyhow::Result<()> {
    conn.execute("INSERT INTO mailing_list (mail_id) VALUES (?)", [mail_id])?;
    Ok(())
}
