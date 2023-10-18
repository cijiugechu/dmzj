use dmzj::Api;

#[tokio::main]
async fn main() {
    let api = Api::new();

    let results = api.search_manga("天国", 0).await.unwrap();

    println!("{:?}", results);
}
