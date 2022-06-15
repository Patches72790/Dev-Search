use dotenv_codegen::dotenv;

pub fn make_searchpage_url(query: &str) -> String {
    let searchpage_url = dotenv!("SEARCHPAGE_URL");
    let context_id = dotenv!("SEARCH_ENGINE_ID");

    format!("{searchpage_url}/cse?cx={context_id}&q={query}")
}

pub fn make_api_url(query: &str) -> String {
    let api_key = dotenv!("SEARCH_API_KEY");
    let context_id = dotenv!("SEARCH_ENGINE_ID");

    format!(
        "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}",
        api_key, context_id, query
    )
}
