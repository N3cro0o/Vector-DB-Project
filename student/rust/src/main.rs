use chroma::ChromaHttpClient as HttpClient;
use std::error::Error;

#[tokio::main]
async fn main() {
    let client = HttpClient::new(Default::default());
    let collection = match client.create_collection("my_collection", None, None).await {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Error while getting collection: {}", err);
            return;
        }
    };

    let embeddings = vec![vec![0.1, 0.2, 0.3], vec![0.4, 0.5, 0.6]];
    match collection
        .add(vec!["id1".to_string(), "id2".to_string()],
            embeddings,
            Some(vec![
                    Some("This is a document about pineapple".to_string()),
                    Some("This is a document about oranges".to_string()),
                ]),
            None,
            None,
        ).await {
    Ok(_) => println!("Inserted items successfuly."),
    Err(err) => {
            eprintln!("Error while inserting to collection: {}", err);
            return;
        }
    }
    let results = match collection
        .query(vec![vec![0.1, 0.2, 0.3]], Some(2), None, None, None)
        .await {
        Ok(r) => r,
        Err(err) => {
            eprintln!("Error while getting results from collection: {}", err);
            return;
        }
    };
    println!("Result: {:#?}", results);
    if let Err(err) = client.delete_collection("my_collection").await {
        eprintln!("Error while deleting collection: {}", err);
    }
}

