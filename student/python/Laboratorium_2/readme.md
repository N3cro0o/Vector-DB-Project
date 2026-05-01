## Laboratorium 2 - Python

### 5.1. Instalacja kluczowej biblioteki

Aby rozpocząć dalszą pracę, musimy zrozumieć co dokładnie wykonywaliśmy na pierwszych laboratoriach. Korzystaliśmy wówczas z biblioteki chromadb, która posiada wbudowany mechanizm generowania embeddingów. Oznacza to, że nie musieliśmy ręcznie przekształcać tekstu na wektory – Chroma automatycznie wykonywała ten proces „w tle” przy dodawaniu i wyszukiwaniu dokumentów.

Takie podejście jest wygodne na etapie wprowadzenia, jednak ogranicza kontrolę nad procesem reprezentacji tekstu. Nie mamy wpływu na wybór modelu, na jego jakość oraz na sposób kodowania znaczenia tekstu.

W bardziej zaawansowanych zastosowaniach, takich jak systemy wyszukiwania semantycznego czy Retrieval-Augmented Generation (RAG), często wymagane jest użycie własnych modeli embeddingowych. Pozwala to na lepsze dopasowanie modelu do konkretnej dziedziny danych oraz poprawę jakości wyszukiwania.

W tym celu wykorzystamy bibliotekę:

```python
pip install sentence-transformers
```

### 5.2. Embedding, wyszukiwanie semantyczne, jak następuje zamiana tekstu na wektor?

Pewnie zastanawiasz się, co autor ma na myśli używając pojęcia embedding. Jest to technika w uczeniu maszynowym i sztucznej inteligencji, która polega na przekształcaniu danych nienumerycznych takich jak słowa, zdania czy całe dokumenty w dane numeryczne, czyli wektory.

Po co jest nam to przekształcenie? Tak jak wspomniano wcześniej, klasyczne bazy danych są w stanie odnaleźć dokumenty jedynie na podstawie dokładnego dopasowania słów kluczowych. Wyszukiwanie semantyczne natomiast analizuje znaczenie tekstu i stara się znaleźć podobieństwa na podstawie kontekstu oraz zbliżonych pojęć, a nie tylko identycznych słów.

Sam proces zamiany tekstu na wektor wygląda następująco. Tekst dzielony jest na mniejsze części, czyli tzw. tokeny (np. słowa). Następnie model uczenia maszynowego przetwarza te tokeny i na podstawie wcześniejszego treningu na dużych zbiorach danych tekstowych, rozpoznaje zależności pomiędzy nimi. W efekcie powstaje jedna reprezentacja całego zdania w postaci wektora liczb, który opisuje jego znaczenie w przestrzeni wielowymiarowej.

Dzięki takiej reprezentacji możliwe jest porównywanie tekstów poprzez obliczanie ich odległości w przestrzeni wektorowej – im mniejsza jest odległość między wektorami, tym bardziej podobne znaczeniowo są dwa teksty.

### 5.3. Metadane

Metadane to dodatkowe informacje przypisane do dokumentu, które opisują jego cechy, ale nie są jego główną treścią. W przypadku baz wektorowych metadane są przechowywane razem z dokumentem i mogą być wykorzystywane do filtrowania wyników wyszukiwania.

W przeciwieństwie do embeddingów, które reprezentują znaczenie tekstu w postaci wektora liczbowego, metadane są danymi strukturalnymi i definiują atrybuty opisujące dokument.

Dzięki metadanym możliwe jest zawężenie wyników wyszukiwania do określonych warunków.

Przykład zdefiniowania metadanych w pojedyńczym dokumencie:

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
Jeżeli chcemy wykorzystać metadane przy filtrowaniu wyników musimy to zrobić podczas wyszukiwania dokumentów w kolekcji. W tym celu dodajemy dodatkowy parametr funkcji query() - where:

```python
where={"continent": "Europe"}
```

### 5.4. Zadanie dla studentów

* Utwórz kolekcję countries w bazie ChromaDB.
* Dodaj minimum 10 dokumentów opisujących państwa (każdy dokument powinien zawierać opis tekstowy kraju).
* Do każdego dokumentu przypisz metadane: kontynent, stolica, waluta.
* Wygeneruj embeddingi dla wszystkich dokumentów przy użyciu biblioteki sentence-transformers.
* Zapisz dokumenty wraz z embeddingami i metadanymi w kolekcji.
* Wykonaj wyszukiwanie semantyczne na podstawie zapytania tekstowego i znajdź 2 najbardziej podobne do siebie państwa.
* Zastosuj filtrowanie wyników wyszukiwania przy użyciu metadanych np. według kontynentu lub stolicy.
* Porównaj działanie wyszukiwania semantycznego z filtrowaniem po metadanych. Opowiedz prowadzącemu jaką widzisz różnice.

W zadaniu należy wykonać wyszukiwanie semantyczne z wykorzystaniem parametru query_embeddings. Oznacza to, że zapytanie tekstowe musi zostać wcześniej zamienione na embedding (wektor liczbowy) przy użyciu modelu sentence-transformers, a następnie przekazane do funkcji query().

### Do generowania embeddingów proszę wykorzystać model all-MiniLM-L6-v2 z biblioteki sentence-transformers ###
