use crate::db::DB;
use crate::utils::get_title_from_url;
use crate::bookmark::Bookmark;

pub fn execute(url: String, tags: Option<Vec<String>>) {
    let db = DB::open();

    let title = get_title_from_url(&url);

    db.add_bookmark(&title, &url.to_string()).unwrap();
    let id = db.last_insert_row_id();

    let mut curr_tags: Vec<String> = Vec::new();

    if let Some(tags) = tags {
        for t in tags {
            db.add_tag_for_bookmark(id, &t);
            curr_tags.push(t.to_string());
        }
    }

    let bookmark = Bookmark::new(
        id, title, url.to_string(), curr_tags,
    );

    println!("{}", bookmark);
}