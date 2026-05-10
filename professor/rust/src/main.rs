use chroma::ChromaHttpClient as HttpClient;
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};

mod lab1;

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
    let mut model = match TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true)) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("Error while creating local all-MiniLM-L6-v2: {}", err);
            return;
        }
    };
    let result_lab1;
    if !USE_LOCAL_MODEL {
        result_lab1 = lab1::basic_db_handling(&client, &collection, Some(&mut model)).await;        
    }
    else {
        result_lab1 = lab1::basic_db_handling(&client, &collection, None).await;        
    }
    if let Err(err) = result_lab1 {
        eprintln!("{}", err);
        return;
    }
    if let Err(err) = client.delete_collection("my_collection").await {
        eprintln!("Error while deleting collection: {}", err);
    }
}

