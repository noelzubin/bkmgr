use crate::db::DB;

pub fn execute(ids: Vec<i64>, _tags: Option<Vec<String>>) {
    let db = DB::open();

    // ids are set
    if !ids.is_empty() {
        ids.into_iter()
        .for_each(|id| {
            match db.get_bookmark_by_id(id) {
                Some(b) => println!("{}", b),
                None => println!("bookmark not found for id: {}\n", id),
            }
        });
        return;
    }

    db.get_all_bookmark()
        .into_iter()
        .for_each(|b| println!("{}", b));
}