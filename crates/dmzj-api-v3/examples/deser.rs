use dmzj_api_v3::{MangaDetailsData, MangaDetailsInfo, MangaDetailsResponse};

fn main() {
    let info = MangaDetailsInfo {
        id: 22588,
        title: "title".into(),
        subtitle: "subtitle".into(),
        types: "types".into(),
        zone: "zone".into(),
        status: "status".into(),
        last_update_chapter_name: "last_update_chapter_name".into(),
        last_updatetime: 15878780,
        cover: "cover".into(),
        authors: "authors".into(),
        description: "description".into(),
        first_letter: "first_letter".into(),
        direction: "direction".into(),
    };

    let data = MangaDetailsData { info };

    let response = MangaDetailsResponse {
        result: 0,
        msg: "msg".into(),
        data,
    };

    let serialized = serde_json::to_string(&response).unwrap();

    println!("serialized = {:?}", serialized);

    let deserialized: MangaDetailsResponse = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialized);
}
