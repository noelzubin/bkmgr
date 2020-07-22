use webbrowser;
use crate::db::DB;

pub fn execute(ids: Vec<i64>) {
    let db = DB::open();

    for id in ids {
        webbrowser::open(&db.get_url_by_id(id)).unwrap();
    }
}