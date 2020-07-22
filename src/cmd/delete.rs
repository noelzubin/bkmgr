use crate::db::DB;

pub fn execute(ids: Vec<i64>) {
    let db = DB::open();
    ids.into_iter().for_each(|id| db.delete_bookmark(id));
}