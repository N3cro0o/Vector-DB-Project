# Laboratorium 1

## 1. Instalacja lokalna bazy danych.

### 1.1. Instalacja wykorzystując `pip` 

W celu korzystania z Chroma użytkownik musi mieć zainstalowany Python 3.9 lub wersję nowszą na swoim komputerze. Następnie w terminalu przy pomocy pip (“Pip Installs Packages”), czyli domyślnego systemu zarządzania pakietami używanego w języku Python instalujemy bibliotekę chromadb.

```python
pip install chromadb
```

### 1.2. Instalacja manualna

Alternatywnym rozwiązaniem jest ręczne zainstalwoanie systemu zarządzania bazą danych. W tym celu należy użyć komendy: 
* Windows PowerShell:
`iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/chroma-core/chroma/main/rust/cli/install/install.ps1'))`
* cURL (Linux | macos):
`curl -sSL https://raw.githubusercontent.com/chroma-core/chroma/main/rust/cli/install/install.sh | bash`

### 1.3. Konteneryzacja

Ostatni sposób wykorzystuje narzędzie Docker. Dla pełnej izolacji środowiska można wykorzystać oficjalny obraz. Do przygotowania obrazu należy wykorzystać polecenie:
`docker pull chromadb/chroma:1.5.3 && docker run -p 8000:8000 chromadb/chroma:1.5.3`

### Uruchomienie bazy danych

Domyślnie serwer Chroma działa pod portem 8000. Aby uruchomić lokalną instancję ChromaDB należy użyć polecenia:
`chroma run --path ./(nazwa katalogu docelowego)`

## 2. Utworzenie klienta

Pierwszym krokiem podczas pracy z bazą danych Chroma jest utworzenie klienta, czyli obiektu odpowiedzialnego za nawiązanie połączenia z bazą oraz wykonywanie na niej operacji. Klient pełni rolę interfejsu komunikacyjnego pomiędzy programem użytkownika, a bazą danych. Oznacza to, że wszystkie dalsze operacje, takie jak tworzenie kolekcji, dodawanie dokumentów, wyszukiwanie danych czy usuwanie rekordów, będą wykonywane właśnie za jego pośrednictwem.
Można to porównać do połączenia z klasyczną bazą danych, np. PostgreSQL lub MySQL, gdzie przed wykonaniem zapytań SQL również konieczne jest najpierw utworzenie połączenia z serwerem.

Bez utworzenia klienta aplikacja nie ma możliwości wykonywania żadnych operacji na danych.

* Python
```python
chroma_client = chromadb.Client()
```
* Rust
```rust
let client = chroma::ChromaHttpClient::new(Default::default());
```
* TypeScript
  * TypeScript ESM
```typescript
import { ChromaClient } from "chromadb";
const client = new ChromaClient();
```
  * TypeScript CJS
```typescript
const { ChromaClient } = require("chromadb");
const client = new ChromaClient();
```

## 3. Utworzenie kolekcji

Po utworzeniu klienta kolejnym krokiem jest stworzenie kolekcji, czyli miejsca, w którym będą przechowywane dane wektorowe oraz powiązane z nimi dokumenty i metadane.
W bazie Chroma kolekcję można porównać do tabeli w relacyjnych bazach danych. Tak jak tabela przechowuje rekordy w wierszach i kolumnach, tak kolekcja przechowuje dokumenty, ich identyfikatory oraz odpowiadające im osadzenia wektorowe.
Utworzenie kolekcji jest konieczne, ponieważ wszystkie dalsze operacje, takie jak dodawanie dokumentów, wyszukiwanie podobnych treści czy usuwanie danych, wykonywane są właśnie na konkretnej kolekcji.

* Python
```python
collection = chroma_client.create_collection(name="my_collection")
```
* Rust
```rust
let client = chroma::ChromaHttpClient::new(Default::default());
```
* TypeScript
```typescript
const collection = await client.createCollection({
    name: "my_collection"
});
```

## 4. Dodawanie dokumentu do kolekcji

Dokumenty stanowią właściwe dane, na których później wykonywane będzie wyszukiwanie semantyczne. Dodawanie dokumentów odbywa się za pomocą metody `add()`. Użytkownik musi znać i wykorzystywać parametry takie jak documents czy ids. Parametr documents zawiera listę tekstów, które mają zostać zapisane w kolekcji. Każdy element listy reprezentuje jeden dokument. Parametr ids zawiera unikalny identyfikator dla każdego utworzonego dokumentu. Działa on w sposób podobny do klucza głównego w relacyjnych bazach danych. Poniżej przedstawiono przykład dodania dokumentu.

Aplikacja serwera ChromaDB posiada wbudowany model `all-MiniLM-L6-v2`, natomiast bezpośredni dostęp do niego jest niemożliwy. Dlatego należy wykorzystać zewnętrzny model do zamiany teksu na wiązania wektorowe. W przeciwnym przypadku należy generować wiązania ręcznie. Do stworzenia lokalnej instancji modelu można wykorzystać pakiet [fastembed](https://crates.io/crates/fastembed).

* Python
```python
collection.add(
    ids=["id4"],
    documents=[
        "Ucze sie wektorowych baz danych"
    ]
)
```
* Rust
```rust
let embeddings = match model.embed(texts, None);
collection.add(vec!["id1".to_string(), "id2".to_string()],
        embeddings,
        Some(vec![
                Some("This is a document about pineapple".to_string()),
                Some("This is a document about oranges".to_string()),
            ]),
        None,
        None,
    )
```
* TypeScript
```typescript
await collection.add({
    ids: ["id1", "id2"],
    documents: [
        "This is a document about pineapple",
        "This is a document about oranges"
    ]
});
```

## 5. Wyszukiwanie dokumentów w kolekcji

W przeciwieństwie do klasycznych baz danych, gdzie wyszukiwanie opiera się głównie na dokładnym dopasowaniu tekstu, Chroma wykorzystuje wyszukiwanie semantyczne, czyli analizę znaczenia zapytania. Odbywa się to przy pomocy metody `query()`. W środowisku **Python** i **TypeScript** możliwe jest użycie parametru `query_texts` lub `queryTexts` do wyszukania elementów zawierających dany tekst. W środowisku **Rust** niemożliwe jest bezpośrednie wyszukanie tekstu. Metoda `query()` zawiera pole `query_embeddings: Vec<Vec<f32>>` pozwalające na wyszukanie łączeń wektorowych. Więcej o łączeniach wektorowych znajdziesz w następnej instrukcji. 

Paramentr `n_result` lub `nResults` lub `n_results: Option<u32>` służy do określania liczby wyników, które mają zostać zwrócone (są to najbardziej podobne dokumenty do naszego tekstu/tekstów).

* Python
```python
results = collection.query(
    query_texts=["Czego się uczysz"],
    n_results=1
)
print(results)
```
* Rust
```rust
let results = match collection
    .query(vec![vec![0.2, 0.2, 0.2]], Some(1), None, None, None);

println!("Result: {:#?}", results);
```
* TypeScript
```typescript
const results = await collection.query({
    queryTexts: ["This is a query about fruit"],
    nResults: 2
});
console.log(results);
```

## 6. Usuwanie danych w Chroma

Podczas pracy z bazą danych często zachodzi potrzeba usunięcia nieaktualnych lub błędnie dodanych danych. W środowisku Chroma możliwe jest usuwanie danych na dwóch poziomach:
* pojedyńczych rekordów,
* całej kolekcji
Usunięcie pojedyńczego rekordu to usunięcie dokumentu o konkretnym identyfikatorze:

* Python
```python
collection.delete(ids["id1"])
```
* Rust
```rust
let response = collection.delete(Some(vec!["id1".to_string()]), None, None);
```
* TypeScript
```typescript
await collection.delete({
    ids: ["id1"]
});
```

Jeżeli chcemy usunąć wszystkie dokumenty i strukturę kolekcji używamy:

* Python
```python
chroma_client.delete_collection(name="my-collection")
```
* Rust
```rust
client.delete_collection("my_collection");
```
* TypeScript
```typescript
await client.deleteCollection({
    name: "my_collection"
});
```

## 7. Źródła

[Chroma Official Docs: Getting Started](https://docs.trychroma.com/docs/overview/getting-started)

[Chroma Cookbook: Installation Guide](https://cookbook.chromadb.dev/core/install/)

[Rust chroma documentation](https://docs.rs/chroma/latest/chroma/index.html)
