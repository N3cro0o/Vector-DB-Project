use chroma::ChromaHttpClient as HttpClient;
use chroma::types;
use std::error::Error;
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};


static USE_LOCAL_MODEL: bool = true;

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
    if (!USE_LOCAL_MODEL) {
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
            .query(vec![vec![0.2, 0.2, 0.2]], Some(2), None, None, None)
            .await {
            Ok(r) => r,
            Err(err) => {
                eprintln!("Error while getting results from collection: {}", err);
                return;
            }
        };
        println!("Result: {:#?}", results);

        let where_struct = types::Where::Document(
            types::DocumentExpression
            {
                operator: types::DocumentOperator::Contains,
                pattern: "orange".to_string(),
            }
        );
        let results = match collection
            .query(vec![vec![0.2, 0.2, 0.2]], Some(2), Some(where_struct), None, None)
            .await {
            Ok(r) => r,
            Err(err) => {
                eprintln!("Error while getting results from collection: {}", err);
                return;
            }
        };
        println!("Result: {:#?}", results);   
    }
    // Text embeding here
    else {
        let mut model = match TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true)) {
            Ok(m) => m,
            Err(err) => {
                eprintln!("Error while creating local all-MiniLM-L6-v2: {}", err);
                return;
            }
        };

        let texts = vec!["This is a document about pineapples".to_string(), "This is a document about oranges".to_string()];
        let embeddings = match model.embed(texts, None) {
            Ok(em) => em,
            Err(err) => {
                eprintln!("Error while generating embeddings: {}", err);
                return;
            }
        };
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
        let query_text = match model.embed(vec!["I fancy pineapples".to_string()], None) {
            Ok(em) => em,
            Err(err) => {
                eprintln!("Error while generating embeddings: {}", err);
                return;
            }
        };

        let results = match collection
            .query(query_text, Some(2), None, None, None)
            .await {
            Ok(r) => r,
            Err(err) => {
                eprintln!("Error while getting results from collection: {}", err);
                return;
            }
        };
        println!("Result: {:#?}", results);
    }
    if let Err(err) = client.delete_collection("my_collection").await {
        eprintln!("Error while deleting collection: {}", err);
    }
}

