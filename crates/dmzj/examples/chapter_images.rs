use dmzj::Api;

#[tokio::main]
async fn main() {
    let api = Api::new();

    let chapter_images = api.fetch_chapter_images(71857, 146271).await.unwrap();

    println!("{:?}", chapter_images.data);
}
