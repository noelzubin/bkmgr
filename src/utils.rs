use select::document::Document;
use select::predicate::Name;

pub fn get_title_from_url(url: &str) -> String {
    let res = reqwest::blocking::get(url).expect("invalid URL").text().unwrap();
    let doc = Document::from(res.as_str());
    doc.find(Name("title")).next().unwrap().text()
}