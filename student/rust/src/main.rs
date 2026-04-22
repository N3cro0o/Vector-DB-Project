use chroma::ChromaHttpClient as HttpClient;
use std::error::Error;

#[tokio::main]
async fn main() {
    let client = HttpClient::new(Default::default());
    let collection = client.create_collection("my_collection", None, None).await.unwrap();

    let embeddings = vec![vec![0.1, 0.2, 0.3], vec![0.4, 0.5, 0.6]];
    collection
    .add(vec!["id1".to_string(), "id2".to_string()],
        embeddings,
        Some(vec![
                Some("This is a document about pineapple".to_string()),
                Some("This is a document about oranges".to_string()),
            ]),
        None,
        None,)
        .await.unwrap();
    let results = collection
        .query(vec![vec![0.1, 0.2, 0.3]], Some(2), None, None, None)
        .await.unwrap();
    println!("Result: {:#?}", results);
    let collection = client.delete_collection("my_collection").await.unwrap();
}

