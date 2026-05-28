import chromadb
from fastembed import TextEmbedding
import ollama

OLLAMA_MODEL = "llama3"

# 1. Wczytanie pliku z dokumentem

def load_markdown(file_path: str) -> str:
    with open(file_path, "r", encoding="utf-8") as f:
        return f.read()
    
# 2. Setup - inicjalizacja klienta ChromaDB

embedder = TextEmbedding()
client = chromadb.Client()

# 3. Todo: Utworzenie kolekcji ChromaDB

collection = None

# 4. Todo: Chunking, czyli podział dużego tekstu na mniejsze fragmenty (chunki), które później są zapisywane w ChromaDB.

def chunk_text(text: str):
    
    """
    TODO:
    Podziel dokument markdown na mniejsze fragmenty.

    """

    pass

# 5. Todo: Zzbuduj indeks w bazie wektorowej:

def build_index(md_text: str):
    
    """
    TODO:
    1. Utwórz kolekcję ChromaDB
    2. Wygeneruj embeddingi
    3. Dodaj dokumenty do bazy wektorowej
    """

    pass

# 6. Todo: Retrieval - wyszukaj podobne fragmenty:

def retrieve(query: str, k: int = 3):
    
    """
    TODO:
    1. Wygeneruj embedding pytania
    2. Wyszukaj najbardziej podobne fragmenty
    """

    pass

# 7. Wygeneruj odpowiedź przez model językowy na podstawie dokumentów znalezionych w bazie wektorowej.

def ask_hr_bot(question: str):
    
    """
    TODO:
    1. Pobierz kontekst z retrieve()
    2. Zbuduj prompt
    3. Wywołaj model llama3
    """

    pass

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