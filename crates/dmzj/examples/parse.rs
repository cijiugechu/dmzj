use dmzj::Api;

#[tokio::main]
async fn main() {
    let api = Api::new();

    let popular_manga = api.fetch_latest_updates_manga(1).await.unwrap();
    let first = &popular_manga[0];
    let first_id = first.id;

    println!("manga id = {:?}", first_id);

    let response = api.fetch_manga_details(first_id).await.unwrap();

    let description = &response.Data.Description;
    let title = &response.Data.Title;
    let cover = &response.Data.Cover;
    let authors = &response.Data.Authors;
    let chapter_ids: &Vec<i32> = &response
        .Data
        .Chapters
        .iter()
        .map(|c| c.Data[0].ChapterId)
        .collect();

    println!("title = {}", title);
    println!("description = {}", description);
    println!("cover = {}", cover);
    println!("authors = {:?}", authors);
    println!("chapters = {:?}", chapter_ids);
}
