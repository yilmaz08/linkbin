use rusqlite::{Connection, Result, params};

const DEFAULT_DB_PATH: &str = "./database.sqlite";

#[derive(Debug)]
pub struct Link {
    pub id: i32,
    pub name: String,
    pub target: String,
}

#[derive(Debug)]
pub struct File {
    pub id: i32,
    pub name: String,
    pub data: Option<Vec<u8>>
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

// UPDATE LINK
pub fn update_link(conn: &Connection, link: &mut Link, new_target: String) -> Result<()> {
    link.target = new_target;
    conn.execute(
        "update links set target=?1 where id=?2",
        params!(&link.target, link.id)
    )?;
    return Ok(());
}

// CREATE FILE
pub fn create_file(conn: &Connection, name: Option<String>, data: Vec<u8>) -> Result<File> {
    let mut file = File {
        id: 0,
        name: name.unwrap(),
        data: Some(data)
    };
    file.id = create_file_db(conn, &file)?;
    return Ok(file);
}

fn create_file_db(conn: &Connection, file: &File) -> Result<i32> {
    let mut stmt = conn.prepare("insert into files (name, data) values (?1, ?2) returning id;")?;
    let mut rows = stmt.query(params![file.name, file.data])?;
    return Ok(rows.next()?.unwrap().get(0)?);
}

// GET FILE
pub fn get_file_by_name(conn: &Connection, name: String) -> Result<Option<File>> {
    let mut stmt = conn.prepare("select id, name from files where name=?1")?;
    let mut rows = stmt.query(params![name])?;
    if let Some(row) = rows.next()? {
        let file = File {
            id: row.get(0)?,
            name: row.get(1)?,
            data: None
        };
        return Ok(Some(file));
    }
    return Ok(None);
}

pub fn get_file_data(conn: &Connection, file: &File) -> Result<Option<Vec<u8>>> {
    let mut stmt = conn.prepare("select data from files where id=?1")?;
    let mut rows = stmt.query(params![&file.id])?;

    if let Some(row) = rows.next()? {
        return Ok(row.get(0)?);
    }
    return Ok(None);
}

// DELETE FILE
pub fn delete_file(conn: &Connection, file: &File) -> Result<()> {
    conn.execute(
        "delete from files where id=?1",
        params!(file.id)
    )?;
    return Ok(());
}

// TABLES and STRUCTURE
pub fn fix_tables(conn: &Connection) -> Result<()> {
    create_table_links(&conn)?;
    create_table_files(&conn)?;
    return Ok(());
}

fn create_table_files(conn: &Connection) -> Result<()> {
    match conn.prepare("SELECT * FROM files;") {
        Ok(..) => {}
        Err(_) => {
            conn.execute(
                "create table files (
                    id integer primary key,
                    name text not null unique,
                    data blob not null
                );",
                ()
            )?;
        }
    }

    return Ok(());
}

fn create_table_links(conn: &Connection) -> Result<()> {
    match conn.prepare("SELECT * FROM links;") {
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
