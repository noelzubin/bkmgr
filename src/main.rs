use structopt::StructOpt;

mod db;
mod bookmark;
mod cmd;
mod utils;

#[derive(Debug, StructOpt)]
enum Opt {
    /// Add a new bookmark
    Add {
        url: String,

        #[structopt(short, long)]
        tags: Option<Vec<String>>
    },

    /// Delete bookmarks
    Delete {
        id: Vec<i64>
    },

    /// List all bookmarks
    List {
        id: Vec<i64>,

        #[structopt(short, long)]
        tags: Option<Vec<String>>
    },

    /// Open bookmark in default browser
    Open { id: Vec<i64> },

    /// Search bookmarks
    Search { keywords: Vec<String> },

    /// Open interactive mode
    Prompt,
}

fn main() {
    let opts = Opt::from_args();
    match opts {
        Opt::Add { url, tags } => cmd::add::execute(url, tags),
        Opt::Delete { id} => cmd::delete::execute(id),
        Opt::List { id, tags} => cmd::list::execute(id, tags),
        Opt::Search { keywords} => cmd::search::execute(keywords),
        Opt::Open { id } => cmd::open::execute(id),
        Opt::Prompt => cmd::prompt::execute().unwrap(),
    }
}
