## Laboratorium 2 - Generowanie osadzeń i metadane

### 1. Instalacja Kluczowych Bibliotek

Podczas pierwszego laboratorium wykorzystywana była wbudowana w bibliotekę `chromadb` funkcja generowania osadzeń wektorowych. Proces transformacji tekstu na wektory był realizowany automatycznie przez bazę danych w tle, oznacza to, że nie musieliśmy ręcznie przekształcać tekstu na wektory. Podejście to, choć stanowi dogodne rozwiązanie na etapie wprowadzającym, znacząco ogranicza kontrolę programisty nad wyborem modelu językowego oraz jakością reprezentacji wektorowej tekstu.

W zaawansowanych systemach wyszukiwania semantycznego czy architekturach **RAG (Retrieval-Augmented Generation)**, standardem jest wykorzystywanie zewnętrznych bibliotek do generowania osadzeń *(embeddings)*. Pozwala to na optymalizację procesu i dobranie modelu idealnie pasującego do charakterystyki danych.

W ramach niniejszych zajęć wykorzystana zostanie biblioteka **[FastEmbed](https://qdrant.github.io/fastembed/)**, opracowana przez zespół inżynierów Qdrant.

W zależności od wybranego środowiska programistycznego, należy zintegrować z projektem odpowiednie pakiety oprogramowania.

##### pip

```python
pip install fastembed
```

##### cargo

```rust
cargo add fastembed
```

Alternatywnie można dodać linię `fastembed = "5.13.4"` w pliku Cargo.toml.

##### npm

```typescipt
npm install fastembed
```

### 2. Osadzanie wektorowe i wyszukiwanie semantyczne

Kluczowym pojęciem w pracy z wektorowymi bazami danych jest **osadzenie (ang. embedding)**. Jest to technika z dziedziny uczenia maszynowego polegająca na transformacji danych nienumerycznych (takich jak słowa, zdania, czy całe dokumenty) do postaci wektorów w przestrzeni wielowymiarowej.

Zastosowanie tej transformacji jest niezbędne dla realizacji wyszukiwania semantycznego. Klasyczne relacyjne bazy danych opierają się na dokładnym dopasowaniu słów kluczowych. Wyszukiwanie semantyczne analizuje natomiast znaczenie tekstu i identyfikuje powiązania na podstawie kontekstu, a nie tylko identyczności znaków.

Proces wektoryzacji można sprowadzić do następujących etapów:

1. Tekst wejściowy dzielony jest na mniejsze jednostki (tokeny).
2. Wytrenowany model uczenia maszynowego analizuje zależności między tokenami.
3. Wynikiem jest pojedyncza reprezentacja matematyczna opisująca sens zdania.

Dzięki takiej reprezentacji możliwe jest porównywanie tekstów poprzez obliczanie ich odległości w przestrzeni. Im mniejszy dystans między wektorami, tym silniejsze powiązanie semantyczne tekstów. 

#### 2.1 Miary odległości w przestrzeni wektorowej

ChromaDB udostępnia następujące metryki obliczania dystansu:
- **Squared L2 (Kwadrat odległości euklidesowej)** – domyślna metryka implementowana przez ChromaDB. Definiuje ona dystans w linii prostej między punktami w przestrzeni wielowymiarowej. Mniejsza wartość odległości euklidesowej koreluje z wyższym podobieństwem semantycznym analizowanych rekordów.

- **Cosine Similarity (Podobieństwo kosinusowe)** – miara powszechnie stosowana w zadaniach przetwarzania języka naturalnego (NLP). Oblicza cosinus kąta zawartego między dwoma wektorami, całkowicie ignorując ich moduł (długość). Stanowi optymalny wybór przy porównywaniu dokumentów o zróżnicowanej objętości tekstu.

- **Inner Product (Iloczyn skalarny)** – stosowana w środowiskach, gdzie generowane wektory są poddawane uprzedniej normalizacji.

#### 2.2 Konfiguracja miary odległości w kodzie źródłowym

Wybór miary odległości definiowany jest na etapie tworzenia kolekcji poprzez przekazanie dedykowanego parametru konfiguracyjnego `hnsw:space` w obiekcie metadanych. W przypadku braku jawnej deklaracji, środowisko przyjmuje wartość `l2` (kwadrat odległości euklidesowej). W kontekście niniejszego ćwiczenia laboratoryjnego domyślna miara będzie wystarczająca, jednak znajomość mechanizmu jej zmiany jest kluczowa przy projektowaniu wydajnych systemów.

### 3. Metadane i filtrowanie

Metadane to dodatkowe informacje strukturalne przypisane do dokumentu, które opisują jego atrybuty (np. autor, data, kategoria)n ale nie są jego główną treścią. Są one przechowywane wraz z dokumentem, a ich głównym zadaniem jest umożliwienie precyzyjnego filtrowania wyników.

Zastosowanie metadanych umożliwia implementację zapytań hybrydowych. Łączą one wyszukiwanie semantyczne z [operatorami logicznymi](https://docs.trychroma.com/docs/querying-collections/metadata-filtering), co pozwala na znaczne zawężenie przestrzeni przeszukiwań i zwiększenie precyzji zwracanych rezultatów.

#### 3.1 Przypisywanie metadanych podczas dodawania dokumentów

Poniżej przedstawiono implementację dodawania dokumentów, połączonych z wcześniej wygenerowanymi wektorami oraz odpowiadającymi im metadanymi.

##### Python

```python
collection.add(
    ids=["1"],
    documents=[
        "Germany is a country in Europe with a strong economy."
    ],
    metadatas=[
        {
            "category": "geography",
            "continent": "Europe",
            "capital": "Berlin",
            "year": 2024
        }
    ]
)
```

##### Rust

```rust
let mut metadata_vec: Vec<Option<HashMap<String, MetadataValue>>> = vec![];
metadata_vec.push(Some(HashMap::from([
    ("continent".into(), MetadataValue::Str("Europe".into())),
    ("capital".into(), MetadataValue::Str("Berlin".into())),
    ("year".into(), MetadataValue::Int(2024))
])));

match collection.add(
    ids_vec,
    embeddings,       
    Some(documents_vec),
    None,
    Some(metadata_vec)
).await;
```

##### TypeScript

```typescript
await collection.add({
    ids: ["1"],
    documents: ["Germany is a country in Europe with a strong economy."],
    embeddings: embeddings, 
    metadatas: [
        {
            continent: "Europe",
            capital: "Berlin",
            year: 2024
        }
    ]
});
```

#### 3.2 Filtrowanie wyników z użyciem parametru `where`

W celu ewaluacji tylko tych dokumentów, które spełniają zdefiniowane parametry, należy dostarczyć obiekt filtrujący w momencie definiowania zapytania.

##### Python

```python
results = collection.query(
    query_embeddings=query_embedding,
    n_results=5,
    where={"continent": "Europe"}
)
```

##### Rust

```rust
let results = collection.query(
    query_embedding,
    Some(5),
    Some(Where::Metadata(MetadataExpression {
        key: "continent".to_string(),
        comparison: MetadataComparison::Primitive(
            PrimitiveOperator::Equal,
            MV::Str("Europe".to_string()),
        )
    })),
    None,
    None
).await;
```

##### TypeScript

```typescript
const results = await collection.query({
    queryEmbeddings: [queryEmbedding],
    nResults: 5,
    where: { continent: "Europe" }
});
```

### 4. Zadania dla studentów

1. Utwórz kolekcję countries w bazie ChromaDB.
2. Dodaj minimum 10 dokumentów opisujących państwa (każdy dokument powinien zawierać opis tekstowy kraju).
3. Do każdego dokumentu przypisz metadane: kontynent, stolica, waluta.
4. Wygeneruj embeddingi dla wszystkich dokumentów przy użyciu biblioteki fastembed.
5. Zapisz dokumenty wraz z embeddingami i metadanymi w kolekcji.
6. Wykonaj wyszukiwanie semantyczne na podstawie zapytania tekstowego i znajdź 2 najbardziej podobne znaczeniowo państwa względem zapytania.
7. Zastosuj filtrowanie wyników wyszukiwania przy użyciu metadanych np. według kontynentu lub stolicy.
8. Porównaj działanie wyszukiwania semantycznego z filtrowaniem po metadanych. Opowiedz prowadzącemu jaką widzisz różnicę.

W zadaniu należy wykonać wyszukiwanie semantyczne z wykorzystaniem parametru `query_embeddings`. Oznacza to, że zapytanie tekstowe musi zostać wcześniej zamienione na embedding (wektor liczbowy) przy użyciu biblioteki `fastembed` i klasy `TextEmbedding()`, a następnie przekazane do funkcji `query()`.
