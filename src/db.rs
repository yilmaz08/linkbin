use rusqlite::{Connection, Result, params};

const DEFAULT_DB_PATH: &str = "./database.sqlite";

#[derive(Debug)]
pub struct Link {
    pub id: i32,
    pub name: String,
    pub target: String,
}

// CONNECTION
pub fn connect(path: Option<&str>) -> Result<Connection> {
    let path = path.unwrap_or(DEFAULT_DB_PATH);
    let conn = Connection::open(path)?;
    fix_tables(&conn)?;
    return Ok(conn);
}

// CREATE LINK
pub fn create_link(conn: &Connection, url: String, name: Option<String>) -> Result<Link> {
    let mut link = Link {
        id: 0,
        target: url,
        name: name.unwrap()
    };
    link.id = create_link_db(conn, &link)?;
    return Ok(link);
}

fn create_link_db(conn: &Connection, link: &Link) -> Result<i32> {
    let mut stmt = conn.prepare("insert into links (name, target) values (?1, ?2) returning id;")?;
    let mut rows = stmt.query(params![link.name, link.target])?;
    return Ok(rows.next()?.unwrap().get(0)?);
}

// GET LINK
pub fn get_link_by_name(conn: &Connection, name: String) -> Result<Option<Link>> {
    let mut stmt = conn.prepare("select id, name, target from links where name=?1")?;
    let mut rows = stmt.query(params![name])?;
    if let Some(row) = rows.next()? {
        let link = Link{
            id: row.get(0)?,
            name: row.get(1)?,
            target: row.get(2)?,
        };
        return Ok(Some(link));
    }
    return Ok(None);
}

// DELETE LINK
pub fn delete_link(conn: &Connection, link: &Link) -> Result<()> {
    conn.execute(
        "delete from links where id=?1",
        params!(link.id)
    )?;
    return Ok(());
}

pub fn update_link(conn: &Connection, link: &mut Link, new_target: String) -> Result<()> {
    link.target = new_target;
    conn.execute(
        "update links set target=?1 where id=?2",
        params!(&link.target, link.id)
    )?;
    return Ok(());
}

// TABLES and STRUCTURE
pub fn fix_tables(conn: &Connection) -> Result<()> {
    create_table_links(&conn)?;
    return Ok(());
}

fn create_table_links(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='links';")?;
    let mut rows = stmt.query(())?;
    match rows.next() {
        Ok(..) => {}
        Err(_) => {
            conn.execute(
                "create table links (
	                id integer primary key,
	                name text not null unique,
	                target text not null
                );",
                ()
            )?;
        }
    }
    return Ok(());
}
