use developer_search::make_searchpage_url;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("Usage: dev[eloper]-search [search-string]");
        std::process::exit(1);
    }

    let search_query = &args[1..].join(" ");

    webbrowser::open(&make_searchpage_url(search_query))?;

    Ok(())
}
