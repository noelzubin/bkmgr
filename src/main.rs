use structopt::StructOpt;
use log::LevelFilter;

mod db;
mod bookmark;
mod cmd;
mod utils;

#[derive(Debug, StructOpt)]
enum Opt {
    Add {
        url: String,

        #[structopt(short, long)]
        tags: Option<Vec<String>>
    },
    Delete {
        id: Vec<i64>
    },
    List {
        id: Vec<i64>,

        #[structopt(short, long)]
        tags: Option<Vec<String>>
    },
    Open { id: Vec<i64> },
    Search { keywords: Vec<String> },
    Prompt,
}

fn main() {
    simple_logging::log_to_file("debug.log", LevelFilter::Info);

    let opts = Opt::from_args();
    match opts {
        Opt::Add { url, tags } => cmd::add::execute(url, tags),
        Opt::Delete { id} => cmd::delete::execute(id),
        Opt::List { id, tags} => cmd::list::execute(id, tags),
        Opt::Search { keywords} => cmd::search::execute(keywords),
        Opt::Open { id } => cmd::open::execute(id),
        Opt::Prompt => cmd::prompt::execute().unwrap(),
        _ => {},
    }
}
