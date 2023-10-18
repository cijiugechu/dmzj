use dmzj::Api;

#[tokio::test]
async fn fetch_popular_manga() {
    let api = Api::new();
    let popular_manga = api.fetch_popular_manga(0).await.unwrap();

    assert!(!popular_manga.is_empty());
}

#[tokio::test]
async fn fetch_latest_updates_manga() {
    let api = Api::new();
    let popular_manga = api.fetch_latest_updates_manga(0).await.unwrap();

    assert!(!popular_manga.is_empty());
}

#[tokio::test]
async fn fetch_manga_details() {
    let api = Api::new();
    let manga = api.fetch_manga_details(14841).await.unwrap();

    assert_eq!(manga.data.id, 14841);
    assert_eq!(manga.data.title, String::from("狂赌之渊"));
}

#[tokio::test]
async fn fetch_category() {
    let api = Api::new();
    let category = api.fetch_category().await.unwrap();

    assert!(!category.data.is_empty());
}

#[tokio::test]
async fn search_manga() {
    let api = Api::new();
    let results = api.search_manga("小镇", 0).await.unwrap();

    assert!(!results.is_empty());
}
