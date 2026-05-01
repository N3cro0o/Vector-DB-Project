import chromadb
from sentence_transformers import SentenceTransformer


model = SentenceTransformer("all-MiniLM-L6-v2")

client = chromadb.Client()

# Tworzymy kolekcję
collection = client.create_collection(name="countries")


# Dokumenty - opisy państw - będą używane do wyszukiwania semantycznego
documents = [
    "Poland is a country in Central Europe with Warsaw as its capital.",
    "Germany is a European country known for its strong economy.",
    "France is famous for Paris, culture and tourism.",
    "Italy is known for its history, food and architecture.",
    "Spain is located in Southern Europe and is popular for tourism.",
    "United States is a large country in North America with a diverse economy.",
    "Canada is known for its cold climate and high quality of life.",
    "Japan is an island country in Asia with advanced technology.",
    "China is one of the largest countries in the world by population.",
    "Brazil is the largest country in South America with the Amazon rainforest."
]
# Metadane - dodatkowe informacje o dokumentach
metadatas = [
    {"continent": "Europe", "capital": "Warsaw", "currency": "PLN"},
    {"continent": "Europe", "capital": "Berlin", "currency": "EUR"},
    {"continent": "Europe", "capital": "Paris", "currency": "EUR"},
    {"continent": "Europe", "capital": "Rome", "currency": "EUR"},
    {"continent": "Europe", "capital": "Madrid", "currency": "EUR"},
    {"continent": "North America", "capital": "Washington D.C.", "currency": "USD"},
    {"continent": "North America", "capital": "Ottawa", "currency": "CAD"},
    {"continent": "Asia", "capital": "Tokyo", "currency": "JPY"},
    {"continent": "Asia", "capital": "Beijing", "currency": "CNY"},
    {"continent": "South America", "capital": "Brasília", "currency": "BRL"}
]

# Dodanie unikalnego ID do dokumentów
ids = [str(i) for i in range(len(documents))]

# Zamiana tekstów na wektory liczbowe
embeddings = model.encode(documents).tolist()

# Zapis dokumentów,metadanych,embeddingów do bazy
collection.add(
    ids=ids,
    documents=documents,
    metadatas=metadatas,
    embeddings=embeddings
)

# Wyszukiwanie semantyczne państwa o podobnym opisie
query = "country with strong economy in Europe"
query_embedding = model.encode([query]).tolist()

results = collection.query(
    query_embeddings=query_embedding,
    n_results=1
)

print("\n=== wyszukiwanie semantyczne ===")
for doc in results["documents"][0]:
    print("-", doc)

# Semantyczne dopasowanie i filtr (bierzemy pod uwagę tylko państwa europejskie)
results = collection.query(
    query_texts=["country"],
    n_results=5,
    where={"continent": "Europe"}
)

print("\n=== państwa w Europie ===")
for doc in results["documents"][0]:
    print("-", doc)

# Zwraca tylko dokument, gdzie stolica to Tokyo
results = collection.query(
    query_texts=["capital cities"],
    n_results=5,
    where={"capital": "Tokyo"}
)

print("\n=== filtrowanie po stolicy (Tokyo) ===")
for doc in results["documents"][0]:
    print("-", doc)

# Bardziej ogólne zapytanie o duże kraje w Azji
query = "big country with many people in Asia"
query_embedding = model.encode([query]).tolist()

results = collection.query(
    query_embeddings=query_embedding,
    n_results=2
)

print("\n=== Top 2 podobne państwa ===")
for doc in results["documents"][0]:
    print("-", doc)