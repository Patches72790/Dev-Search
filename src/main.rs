use dotenv_codegen::dotenv;

fn main() {
    let api_key = dotenv!("SEARCH_API_KEY");
    let context_id = dotenv!("SEARCH_ENGINE_ID");

    println!("API KEY: {} CONTEXT_ID: {}", api_key, context_id);
}
