use chroma::ChromaHttpClient as HttpClient;
use chroma::ChromaCollection;
use chroma::types::*;
use chroma::types::MetadataValue as MV;
use fastembed::TextEmbedding;

use std::collections::HashMap;

pub async fn embedding_and_metadata(client: &HttpClient, model_borrow: Option<&mut TextEmbedding>) -> Result<(), String> {
    let mut model = match model_borrow {
        Some(m) => m,
        None => { return Err("No model provided".to_string()); }
    };
    
    let collection = match client.create_collection("countries", None, None).await {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Error while getting collection: {}", err);
            return Err(format!("Cannot create collection. {}", err));
        }
    };

    let mut countries_vec: Vec<&str> = vec![];
    countries_vec.push("Poland is a country in Central Europe with Warsaw as its capital.");
    countries_vec.push("Germany is a European country known for its strong economy.");
    countries_vec.push("France is famous for Paris, culture and tourism.");
    countries_vec.push("Italy is known for its history, food and architecture.");
    countries_vec.push("Spain is located in Southern Europe and is popular for tourism.");
    countries_vec.push("United States is a large country in North America with a diverse economy.");
    countries_vec.push("Canada is known for its cold climate and high quality of life.");
    countries_vec.push("Japan is an island country in Asia with advanced technology.");
    countries_vec.push("China is one of the largest countries in the world by population.");
    countries_vec.push("Brazil is the largest country in South America with the Amazon rainforest.");
    
    let mut metadata_vec: Vec<Option<HashMap<String, MV>>> = vec![];
    metadata_vec.push(Some(HashMap::from([
                ("continent".into(), MV::Str("Europe".into())), 
                ("capital".into(), MV::Str("Warsaw".into())),
                ("currency".into(), MV::Str("PLN".into()))])));
    metadata_vec.push(Some(HashMap::from([
                ("continent".into(), MV::Str("Europe".into())),
                ("capital".into(), MV::Str("Berlin".into())),
                ("currency".into(), MV::Str("EUR".into()))])));
    metadata_vec.push(Some(HashMap::from(
                [("continent".into(), MV::Str("Europe".into())),
                ("capital".into(), MV::Str("Paris".into())),
                ("currency".into(), MV::Str("EUR".into()))])));
    metadata_vec.push(Some(HashMap::from([
                ("continent".into(), MV::Str("Europe".into())),
                ("capital".into(), MV::Str("Rome".into())),
                ("currency".into(), MV::Str("EUR".into()))])));
    metadata_vec.push(Some(HashMap::from([
                ("continent".into(), MV::Str("Europe".into())),
                ("capital".into(), MV::Str("Madrit".into())),
                ("currency".into(), MV::Str("EUR".into()))])));
    metadata_vec.push(Some(HashMap::from([
                ("continent".into(), MV::Str("North America".into())),
                ("capital".into(), MV::Str("Washington D.C.".into())),
                ("currency".into(), MV::Str("USD".into()))])));
    metadata_vec.push(Some(HashMap::from([
                ("continent".into(), MV::Str("North America".into())),
                ("capital".into(), MV::Str("Ottawa".into())),
                ("currency".into(), MV::Str("CAD".into()))])));
    metadata_vec.push(Some(HashMap::from([
                ("continent".into(), MV::Str("Asia".into())),
                ("capital".into(), MV::Str("Tokio".into())),
                ("currency".into(), MV::Str("JPY".into()))])));
    metadata_vec.push(Some(HashMap::from([
                ("continent".into(), MV::Str("Asia".into())),
                ("capital".into(), MV::Str("Beijing".into())),
                ("currency".into(), MV::Str("CNY".into()))])));
    metadata_vec.push(Some(HashMap::from([
                ("continent".into(), MV::Str("South America".into())),
                ("capital".into(), MV::Str("Brasília".into())),
                ("currency".into(), MV::Str("BRL".into()))])));
    let mut ids_vec: Vec<String> = vec![];
    for i in 0..countries_vec.len() {
        ids_vec.push(i.to_string());
    }
    let embeddings = match model.embed(countries_vec.clone(), None) {
        Ok(em) => em,
        Err(err) => {
            return Err(format!("Error while generating embeddings: {}", err));
        }
    };
    let some_vec: Vec<Option<String>> = countries_vec.iter().map(|text| Some(text.to_string())).collect();
    match collection
        .add(ids_vec,
            embeddings,
            Some(some_vec),
            None,
            Some(metadata_vec),
        ).await {
    Ok(_) => println!("Inserted items successfuly."),
    Err(err) => {
            return Err(format!("Error while inserting to collection: {}", err));
        }
    }
    let query = "Country with strong economy in Europe".to_string();
    let query = match model.embed(vec![query], None) {
        Ok(em) => em,
        Err(err) => {
            return Err(format!("Error while generating embeddings: {}", err));
        }
    };
    
    // Standard search
    let results = match collection
        .query(query, Some(1), None, None, None)
        .await {
            Ok(r) => r,
            Err(err) => {
                return Err(format!("Error while getting results from collection: {}", err));
            }
        };
    println!("Standard result: {:#?}", results);
    let query = "Country".to_string();
    let query = match model.embed(vec![query], None) {
        Ok(em) => em,
        Err(err) => {
            return Err(format!("Error while generating embeddings: {}", err));
        }
    }; 
    // Only Europe
    let results = match collection
        .query(
            query,
            Some(5),
            Some(Where::Metadata(MetadataExpression {
                        key: "continent".to_string(),
                        comparison: MetadataComparison::Primitive(
                                PrimitiveOperator::Equal,
                                MV::Str("Europe".to_string()),
                            )
                    })
                ),
            None,
            None)
        .await {
            Ok(r) => r,
            Err(err) => {
                return Err(format!("Error while getting results from collection: {}", err));
            }
        };
    println!("Continent result: {:#?}", results);
    let query = "Capital city".to_string();
    let query = match model.embed(vec![query], None) {
        Ok(em) => em,
        Err(err) => {
            return Err(format!("Error while generating embeddings: {}", err));
        }
    }; 
    // Only Tokyo
    let results = match collection
        .query(
            query,
            Some(5),
            Some(Where::Metadata(MetadataExpression {
                        key: "capital".to_string(),
                        comparison: MetadataComparison::Primitive(
                                PrimitiveOperator::Equal,
                                MV::Str("Tokyo".to_string()),
                            )
                    })
                ),
            None,
            None)
        .await {
            Ok(r) => r,
            Err(err) => {
                return Err(format!("Error while getting results from collection: {}", err));
            }
        };
    println!("Tokyo result: {:#?}", results);
    let query = "Big country with large population in Asia".to_string();
    let query = match model.embed(vec![query], None) {
        Ok(em) => em,
        Err(err) => {
            return Err(format!("Error while generating embeddings: {}", err));
        }
    }; 
    // Big countries in Asia
    let results = match collection
        .query(
            query,
            Some(2),
            None,
            None,
            None)
        .await {
            Ok(r) => r,
            Err(err) => {
                return Err(format!("Error while getting results from collection: {}", err));
            }
        };
    println!("Big countires result: {:#?}", results);
    
    Ok(())
}
