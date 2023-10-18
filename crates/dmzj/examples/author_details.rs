use dmzj::Api;

#[tokio::main]
async fn main() {
    let api = Api::new();

    let manga_details = api.fetch_manga_details(14841).await.unwrap();

    let first_author = &manga_details.data.authors[0];

    let first_author_tag_id = first_author.tagId;

    let author_details =
        api.fetch_author_details(first_author_tag_id).await.unwrap();

    println!("{:?}", author_details);
}
