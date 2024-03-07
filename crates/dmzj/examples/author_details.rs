use dmzj::Api;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let api = Api::new();

    let manga_details = api.fetch_manga_details(14841).await.unwrap();

    let first_author = &manga_details.data.authors[0];

    let first_author_tag_id = first_author.tagId;

    let author_details =
        api.fetch_author_details(first_author_tag_id).await.unwrap();

    println!("{:?}", author_details);
}
