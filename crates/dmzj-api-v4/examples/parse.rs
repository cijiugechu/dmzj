use dmzj_api_v4::ApiV4;

#[tokio::main]
async fn main() {
    let api = ApiV4::new();

    let response = api.fetch_manga_details(48194).await.unwrap();

    let description = &response.Data.Description;
    let title = &response.Data.Title;
    let cover = &response.Data.Cover;
    let authors = &response.Data.Authors;

    println!("title = {}", title);
    println!("description = {}", description);
    println!("cover = {}", cover);
    println!("authors = {:?}", authors);
}
