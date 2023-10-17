use dmzj::Api;

#[tokio::test]
async fn fetch_popular_manga() {
    let api = Api::new();
    let popular_manga = api.fetch_popular_manga(1).await.unwrap();

    assert!(!popular_manga.is_empty());
}

#[tokio::test]
async fn fetch_latest_updates_manga() {
    let api = Api::new();
    let popular_manga = api.fetch_latest_updates_manga(1).await.unwrap();

    assert!(!popular_manga.is_empty());
}

#[tokio::test]
async fn fetch_manga_details() {
    let api = Api::new();
    let manga = api.fetch_manga_details(14841).await.unwrap();

    assert_eq!(manga.data.id, 14841);
    assert_eq!(manga.data.title, String::from("狂赌之渊"));
}
