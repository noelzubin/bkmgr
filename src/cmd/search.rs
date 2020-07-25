use crate::db::DB;
use crate::bookmark::Bookmark;

pub fn execute(keywords: Vec<String>) {
    let bookmarks = search(keywords);

    if bookmarks.len() == 0 {
        println!("Error: No matching any bookmark");
        return;
    }

    for bookmark in bookmarks {
        println!("{}", bookmark);
    }
}

pub fn search(keywords: Vec<String>) -> Vec<Bookmark>{ 
    DB::open().search(keywords)
}