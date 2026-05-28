use chroma::ChromaHttpClient as HttpClient;
use chroma::ChromaCollection;
use chroma::types::*;
use chroma::types::MetadataValue as MV;
use fastembed::TextEmbedding;
use ollama_rs::generation::completion::request::GenerationRequest;

const OLLAMA_MODEL: &str = "llama3";

fn load_file(path: &str) -> std::io::Result<String> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut str_buf = String::new();
    file.read_to_string(&mut str_buf)?;
    Ok(str_buf)
}

fn split_file_to_chunks(file: &String) -> Result<Vec<String>, String> {
    let v: Vec<&str> = file.split("\n")
            .filter(|line| !line.is_empty() && *line != "---")
            .collect();
    let mut chunks: Vec<String> = vec![]; // one pos -> one chunk
    let mut curr_chunk: Option<String> = None;

    for l in v.iter() {
        if l.starts_with("##") {
            if let Some(s) = &curr_chunk {
                chunks.push(s.clone());
            }
            
            curr_chunk = Some(String::new());
        }

        if let Some(s) = &mut curr_chunk {
            s.push_str(l);
            s.push('\n');
        }
    }
    
    if let Some(s) = &curr_chunk {
        chunks.push(s.clone());
    }

    Ok(chunks)
}

pub async fn setup(client: &HttpClient, model_borrow: Option<&mut TextEmbedding>, path: &str) -> Result<ChromaCollection, String> {
    let mut preformat = match load_file(path) {
        Ok(s) => s,
        Err(err) => { return Err(err.to_string()); }
    };
    let chunks = match split_file_to_chunks(&mut preformat) {
        Ok(c) => c,
        Err(err) => { return Err(err.to_string()); }
    };

    Err(String::from("This is example function. Finish creating chroma collection."))
}


async fn gen_answer(input: String, model: &mut TextEmbedding, collection: &ChromaCollection, ollama: &ollama_rs::Ollama) -> Result<String, String> {
    Err(String::from("Finish function gen_answer"))
}

pub async fn chat_with_bot(model_borrow: Option<&mut TextEmbedding>, collection: ChromaCollection) -> Result<(), String> {
    let mut model = match model_borrow {
        Some(m) => m,
        None => { return Err("No model provided".to_string()); }
    };
    let ollama = ollama_rs::Ollama::default();
    println!("Type 'exit' to quit chat.");
    loop {
        println!("Ask:");
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        input = input.trim().to_string();
        if input.to_lowercase() == "exit" {
            break;
        }
        let answer = gen_answer(input, model, &collection, &ollama).await.unwrap();
        println!("Bot: {answer}");
    }

    Ok(())
}
