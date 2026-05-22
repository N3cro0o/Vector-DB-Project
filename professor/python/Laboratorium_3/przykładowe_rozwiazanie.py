import chromadb
from fastembed import TextEmbedding
import ollama

# 1. Wczytanie pliku z dokumentem

def load_markdown(file_path: str) -> str:
    with open(file_path, "r", encoding="utf-8") as f:
        return f.read()

embedder = TextEmbedding()

# 2. Utworzenie ChromaDB

client = chromadb.Client()

# 3. Tworzymy kolekcję

collection = client.create_collection(name="hr_policy")


# 4. Chunking, czyli podział dużego tekstu na mniejsze fragmenty (chunki), które później są zapisywane w ChromaDB.

def chunk_text(text: str):

    chunks = []
    current_chunk = []

    for line in text.split("\n"):

        # nowa sekcja po nagłówku #
        if line.startswith("##"):

            if current_chunk:
                chunks.append("\n".join(current_chunk))
                current_chunk = []

        current_chunk.append(line)

    if current_chunk:
        chunks.append("\n".join(current_chunk))

    return [chunk.strip() for chunk in chunks if chunk.strip()]

# 5. Ta funkcja odpowiada za zbudowanie indeksu w bazie wektorowej, czyli:
# podział dokumentu na chunki,
# wygenerowanie embeddingów,
# zapisanie danych do ChromaDB.

def build_index(md_text: str):

    chunks = chunk_text(md_text)
    for i, chunk in enumerate(chunks):

        vector = list(embedder.embed(chunk))[0]

        collection.add(
            documents=[chunk],
            embeddings=[vector],
            ids=[f"doc_{i}"]
        )

    print(f"Dodano {len(chunks)} fragmentów do bazy.")

# 6. wyszukiwanie najbardziej podobnych fragmentów tekstu w bazie wektorowej.

def retrieve(query: str, k: int = 3):

    query_vector = list(embedder.embed(query))[0]

    results = collection.query(
        query_embeddings=[query_vector],
        n_results=k
    )

    return results["documents"][0]

# 7. Generowanie odpowiedzi przez model językowy na podstawie dokumentów znalezionych w bazie wektorowej.

def ask_hr_bot(question: str):

    documents = retrieve(question)
    context = "\n\n".join(documents)

    prompt = f"""
Jesteś asystentem HR.

Odpowiadaj tylko na podstawie podanego kontekstu.

Jeżeli nie ma informacji w kontekście, napisz:
"Brak informacji w dokumentacji."

Kontekst:
{context}

Pytanie:
{question}
"""

    response = ollama.chat(
        model="llama3",
        messages=[
            {
                "role": "user",
                "content": prompt
            }
        ]
    )

    return response["message"]["content"]

# 8. Ta funkcja odpowiada za prosty interfejs tekstowy (CLI), który pozwala użytkownikowi rozmawiać z chatbotem RAG w terminalu.
# Dzięki niej użytkownik może wielokrotnie zadawać pytania bez ponownego uruchamiania programu.

def chat():

    print("\nHR Assistant")
    print("Wpisz 'exit' aby zakończyć.\n")

    while True:

        question = input("Ty: ")
        if question.lower() == "exit":
            break

        answer = ask_hr_bot(question)
        print("\nBot:", answer)
        print()


# 9. Ten fragment odpowiada za uruchomienie całego programu.

if __name__ == "__main__":
    file_name = "dane.md"
    print("Wczytywanie pliku:", file_name)
    text = load_markdown(file_name)
    build_index(text)
    chat()