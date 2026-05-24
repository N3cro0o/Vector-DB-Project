use chroma::ChromaHttpClient as HttpClient;
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};

mod lab1;
mod lab2;
mod lab3;

static USE_LOCAL_MODEL: bool = true;
const LAB_NUM: u32 = 3;
const RAG_FILE: &str = "../dane.md";

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
    match LAB_NUM {
        1 => {
            let result_lab1;
            if USE_LOCAL_MODEL {
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

        2 => {
            if let Err(err) = lab2:: embedding_and_metadata(&client, Some(&mut model)).await{
                eprintln!("{}", err);
                return;
            }
            if let Err(err) = client.delete_collection("countries").await {
                eprintln!("Error while deleting collection: {}", err);
            }
        }

        3 => {
            let coll = lab3::setup(&client, Some(&mut model), RAG_FILE).await.unwrap();
            lab3::chat_with_bot(Some(&mut model), coll).await;
        }

        _ => { eprintln!("Invalid lab number"); } 
    }
}
