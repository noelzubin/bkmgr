use crate::db::DB;
use rusqlite::{ params };
use crate::bookmark::Bookmark;

pub fn execute() {
    let db = DB::open();

    let mut stmt = db.conn.prepare("
        select b.*, group_concat(t.name) from bookmarks b 
        join bookmark_tag bt on bt.bookmark_id = b.id
        join tags t on t.id = bt.tag_id
    ").unwrap();
    stmt.query_map(params![], |row| {
        Ok(Bookmark::new(
            row.get(0).unwrap(),
            row.get(1).unwrap(),
            row.get(2).unwrap(),
            row.get::<_, String>(3).unwrap().split(",").map(String::from).collect(),
        ))
    }).unwrap().for_each(|b| println!("{}", b.unwrap()));

}
