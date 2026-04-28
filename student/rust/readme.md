## Laboratorium 1 - Rust

Przykładowy program należy wpierw skompilować poprzez wpisanie komendy `cargo run` albo poprzez `cargo build` i uruchomienie pliku wykonywalnego w katalogu `target/debug`.

### 1. Lokalna instalacja Chroma
Do poprawnego uruchomienia projektu wykorzystującego lokalną instancję ChromaDB należy ją ręcznie zainstalować. Aby tego dokonać należy wykorzystać narzędzie pip, zainstalować kontener zawierający bazę danych albo ręcznie pobrać wykorzystując link. [Wszystkie opcje opisano w dokumentacji](https://cookbook.chromadb.dev/core/install/#__tabbed_2_2) oraz [w dokumentacji pakietu chroma](https://docs.rs/chroma/latest/chroma/index.html).


W tym katalogu przygotowano przykładowy kod obsługujący bazę danych ChromaDB. Do poprawnego działania programu należy przygotować lokalną instancję bazy danych poprzez wpisanie komendy: 

`chroma run --path ./lab1`

Po zainstalowaniu bazy danych należy dodać niezbędny pakiet do pliku Cargo.toml. Aby tego dokonać należy wpisać komendę:

`cargo add chroma`

#### 2. Utworzenie klienta

Pierwszym krokiem podczas pracy z bazą danych Chroma jest utworzenie klienta, czyli obiektu odpowiedzialnego za nawiązanie połączenia z bazą oraz wykonywanie na niej operacji. Klient pełni rolę interfejsu komunikacyjnego pomiędzy programem użytkownika, a bazą danych. Oznacza to, że wszystkie dalsze operacje, takie jak tworzenie kolekcji, dodawanie dokumentów, wyszukiwanie danych czy usuwanie rekordów, będą wykonywane właśnie za jego pośrednictwem.
Można to porównać do połączenia z klasyczną bazą danych, np. PostgreSQL lub MySQL, gdzie przed wykonaniem zapytań SQL również konieczne jest najpierw utworzenie połączenia z serwerem.

Bez utworzenia klienta aplikacja nie ma możliwości wykonywania żadnych operacji na danych. W środowisku rust strukturę chroma::ChromaHttpClient wygląda następująco:

```rust
let client = HttpClient::new(Default::default());
```

#### 3. Utworzenie kolekcji

Po utworzeniu klienta kolejnym krokiem jest stworzenie kolekcji, czyli miejsca, w którym będą przechowywane dane wektorowe oraz powiązane z nimi dokumenty i metadane.
W bazie Chroma kolekcję można porównać do tabeli w relacyjnych bazach danych. Tak jak tabela przechowuje rekordy w wierszach i kolumnach, tak kolekcja przechowuje dokumenty, ich identyfikatory oraz odpowiadające im osadzenia wektorowe.
Utworzenie kolekcji jest konieczne, ponieważ wszystkie dalsze operacje, takie jak dodawanie dokumentów, wyszukiwanie podobnych treści czy usuwanie danych, wykonywane są właśnie na konkretnej kolekcji.

```rust
let collection = match client.create_collection("my_collection", None, None);
```
Ponieważ metoda `create_collection` zwraca typ `std::future::Future`, nalezy wykorzystać słowo kluczowe `await` albo metodę `poll()`.

#### 4. Dodanie dokumentu do kolekcji
Dokumenty stanowią właściwe dane, na których później wykonywane będzie wyszukiwanie semantyczne. Dodawanie dokumentów odbywa się za pomocą metody add().

Użytkownik musi znać i wykorzystywać parametry takie jak documents czy ids. Parametr documents zawiera listę tekstów, które mają zostać zapisane w kolekcji. Każdy element listy reprezentuje jeden dokumen. 

Parametr ids zawiera unikalny identyfikator dla każdego utworzonego dokumentu. Działa on w sposób podobny do klucza głównego w relacyjnych bazach danych. Poniżej przedstawiono przykład dodania dokumentu.

```python
collection.add(
    vec!["id1".to_string(), "id2".to_string()],
    embeddings,
    Some(vec![
        Some("This is a document about pineapple".to_string()),
        Some("This is a document about oranges".to_string()),
    ]),
    None,
    None);
```

#### 5. Wyszukiwanie dokumentów w kolekcji
W przeciwieństwie do klasycznych baz danych, gdzie wyszukiwanie opiera się głównie na dokładnym dopasowaniu tekstu, Chroma wykorzystuje wyszukiwanie semantyczne, czyli analizę znaczenia zapytania. Odbywa się to przy pomocy metody query().

Tak samo jak w przypadku dodawania dokumentów trzeba znać istotne parametry takie jak `query_embeddings: Vec<Vec<f32>>` i `n_results: Option<u32>`. Query_embeddings zawiera listę powiązań wektorowych wykorzystywanych, które mają zostać znalezione. Chroma umożliwia wyszukiwanie kilku zapytań jednocześnie.

n_results natomiast służy do określania liczby wyników, które mają zostać zwrócone (są to najbardziej podobne dokumenty do naszego tekstu/tekstów).

```rust
let results = match collection
    .query(vec![vec![0.2, 0.2, 0.2]], Some(1), None, None, None);

println!("Result: {:#?}", results);
```

Dodatkowe opcje wykszukiwania są dostępne poprzez strukturę `chroma::types::Where`.

#### 6. Usuwanie danych w Chroma
Podczas pracy z bazą danych często zachodzi potrzeba usunięcia nieaktualnych lub błędnie dodanych danych. W środowisku Chroma możliwe jest usuwanie danych na dwóch poziomach:
* pojedyńczych rekordów,
* całej kolekcji
Usunięcie pojedyńczego rekordu to usunięcie dokumentu o konkretnym identyfikatorze:

```rust
let response = collection.delete(Some(vec!["id1".to_string()]), None, None);
```

Jeżeli chcemy usunąć wszystkie dokumenty i strukturę kolekcji używamy:

```rust
client.delete_collection("my_collection");
```

Analogiczną operacją wykorzystywaną w klasycznych bazach danych jest DROP TABLE.
