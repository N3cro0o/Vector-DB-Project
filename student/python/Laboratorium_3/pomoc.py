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

# 3. Ta funkcja odpowiada za prosty interfejs tekstowy (CLI), który pozwala użytkownikowi rozmawiać z chatbotem RAG w terminalu.
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


# 4. Ten fragment odpowiada za uruchomienie całego programu.

if __name__ == "__main__":
    file_name = "dane.md"
    print("Wczytywanie pliku:", file_name)
    text = load_markdown(file_name)
    build_index(text)
    chat()
