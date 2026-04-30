use chroma::ChromaHttpClient as HttpClient;
use chroma::ChromaCollection;
use chroma::types;
use fastembed::TextEmbedding;

pub async fn basic_db_handling(_client: &HttpClient, collection: &ChromaCollection, model_borrow: Option<&mut TextEmbedding>) -> Result<(), String> {
    if let Some(model) = model_borrow {
        let texts = vec!["This is a document about pineapples".to_string(), "This is a document about oranges".to_string()];
        let embeddings = match model.embed(texts, None) {
            Ok(em) => em,
            Err(err) => {
                return Err(format!("Error while generating embeddings: {}", err));
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
                return Err(format!("Error while inserting to collection: {}", err));
            }
        }
        let query_text = match model.embed(vec!["I fancy pineapples".to_string()], None) {
            Ok(em) => em,
            Err(err) => {
                return Err(format!("Error while generating embeddings: {}", err));
            }
        };

        let results = match collection
            .query(query_text, Some(2), None, None, None)
            .await {
            Ok(r) => r,
            Err(err) => {
                return Err(format!("Error while getting results from collection: {}", err));
            }
        };
        println!("Result: {:#?}", results);
    }
    else {
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
                return Err(format!("Error while inserting to collection: {}", err));
            }
        }
        let results = match collection
            .query(vec![vec![0.2, 0.2, 0.2]], Some(2), None, None, None)
            .await {
            Ok(r) => r,
            Err(err) => {
                return Err(format!("Error while getting results from collection: {}", err));
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
                return Err(format!("Error while getting results from collection: {}", err));
            }
        };
        println!("Result: {:#?}", results);   
    }
    Ok(())
}
