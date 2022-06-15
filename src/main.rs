use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() {
    let api_key = dotenv!("SEARCH_API_KEY");
    let context_id = dotenv!("SEARCH_ENGINE_ID");

    let url_string = format!(
        "https://www.googleapis.com/customsearch/v1?key={}&cx={}",
        api_key, context_id
    );


}
