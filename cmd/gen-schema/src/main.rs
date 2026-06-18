use app::routes::ApiDoc;
use utoipa::OpenApi;

fn main() {
    let doc = ApiDoc::openapi().to_pretty_json().expect("serialize openapi");
    println!("{doc}");
}
