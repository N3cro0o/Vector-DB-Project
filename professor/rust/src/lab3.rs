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

async fn gen_collection(client: &HttpClient, model_borrow: Option<&mut TextEmbedding>, chunks: Vec<String>) -> Result<ChromaCollection, String> {
    let mut model = match model_borrow {
        Some(m) => m,
        None => { return Err("No model provided".to_string()); }
    };

    let collection = match client.create_collection("hr_policy", None, None).await {
        Ok(c) => c,
        Err(err) => {
            return Err(format!("Error while getting collection: {}", err));
        }
    };

    let mut i = 0;
    let mut doc_vec: Vec<Option<String>> = vec![];
    let mut id_vec: Vec<String> = vec![];
    let embed_vec = match model.embed(&chunks, None) {
        Ok(em) => em,
        Err(err) => { return Err(err.to_string()); }
    };
    for c in chunks.iter(){
        id_vec.push(format!("policy_{i}"));
        doc_vec.push(Some(c.clone()));
        i += 1;
    }

    match collection
        .add(id_vec, embed_vec, Some(doc_vec), None, None).await {
        Ok(_) => println!("Inserted items successfuly."),
        Err(err) => {
            return Err(format!("Error while inserting to collection: {}", err));
        }
    };
    
    Ok(collection)
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
    let c = match gen_collection(client, model_borrow, chunks).await {
        Ok(coll) => coll,
        Err(err) => { return Err(err.to_string()); }
    };
    let count = c.count().await.unwrap();
    println!("Collection contains {} records", count);    

    Ok(c)
}

async fn gen_answer(input: String, model: &mut TextEmbedding, collection: &ChromaCollection, ollama: &ollama_rs::Ollama) -> Result<String, String> {
    let mut answ = String::new();

    let embed = match model.embed(vec![&input], None) {
        Ok(em) => em,
        Err(err) => { return Err(err.to_string()); }
    };
    let results = match collection.query(embed, Some(3), None, None, None).await {
        Ok(em) => em,
        Err(err) => {
            return Err(format!("Error while generating embeddings: {}", err));
        }
    };
    let docs = results.documents.unwrap();
    for document in docs[0].iter(){
        let line = document.clone().unwrap();
        answ.push_str(&line);
        answ.push_str("\n\n");
    }
    
    let prompt = format!("Jesteś asystentem HR. Odpowiadaj tylko na podstawie podanego kontekstu.
        Jeżeli nie ma informacji w kontekście, napisz: \"Brak informacji w dokumentacji.\"
        Kontekst:{}
        Pytanie:{}", answ, input);
    answ = match ollama.generate(GenerationRequest::new(OLLAMA_MODEL.to_string(), prompt)).await {
        Ok(a) => a.response,
        Err(err) => { return Err(err.to_string()); }
    };

    Ok(answ)
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
