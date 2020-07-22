use crate::db::DB;

pub fn execute(keywords: Vec<String>) {
    let db = DB::open();

    let bookmarks = db.search(keywords);

    if bookmarks.len() == 0 {
        println!("Error: No matching any bookmark");
        return;
    }

    for bookmark in bookmarks {
        println!("{}", bookmark);
    }
}