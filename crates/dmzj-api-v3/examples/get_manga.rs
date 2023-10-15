use dmzj_api_v3::ApiV3;

#[tokio::main]
async fn main() {
    let api = ApiV3::new();

    let popular_manga = api.fetch_latest_updates_manga(2).await.unwrap();
    let first = &popular_manga[7];
    let first_id = first.id;

    println!("{:?}", first_id);

    let manga_details = api.fetch_manga_details(first_id).await.unwrap();

    println!("{:?}", manga_details);
}
