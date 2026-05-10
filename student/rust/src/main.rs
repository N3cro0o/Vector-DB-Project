use chroma::{ChromaHttpClient, ChromaCollection};
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};

#[tokio::main]
async fn main() {
    // To create embeddings use model.embed('AsRef<str>', None) -> Result<Vec<Embedding>>
    let mut model = match TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true)) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("Error while creating local all-MiniLM-L6-v2: {}", err);
            return;
        }
    };

    // Add your code here
}
