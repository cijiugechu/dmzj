use dmzj::Api;

#[tokio::main]
async fn main() {
    let api = Api::new();

    let category = api.fetch_category().await.unwrap();

    println!("{:?}", category);
}
