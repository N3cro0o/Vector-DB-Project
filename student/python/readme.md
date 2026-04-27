## Laboratorium 1 - Python
### 4.1. Lokalna instalacja Chroma

W celu korzystania z Chroma użytkownik musi mieć zainstalowany Python 3.9 lub wersję nowszą na swoim komputerze.
W celu sprawdzenia wersji Pythona w PyCharm użytkownik musi wykonać następujące kroki w programie:

`File → Settings → Project → Python Interpreter`

Następnie w terminalu przy pomocy pip (“Pip Installs Packages”), czyli domyślnego systemu zarządzania pakietami używanego w języku Python instalujemy bibliotekę chromadb.

`pip install chromadb`

#### 4.2. Utworzenie klienta

Pierwszym krokiem podczas pracy z bazą danych Chroma jest utworzenie klienta, czyli obiektu odpowiedzialnego za nawiązanie połączenia z bazą oraz wykonywanie na niej operacji. Klient pełni rolę interfejsu komunikacyjnego pomiędzy programem użytkownika, a bazą danych. Oznacza to, że wszystkie dalsze operacje, takie jak tworzenie kolekcji, dodawanie dokumentów, wyszukiwanie danych czy usuwanie rekordów, będą wykonywane właśnie za jego pośrednictwem.
Można to porównać do połączenia z klasyczną bazą danych, np. PostgreSQL lub MySQL, gdzie przed wykonaniem zapytań SQL również konieczne jest najpierw utworzenie połączenia z serwerem.

Bez utworzenia klienta aplikacja nie ma możliwości wykonywania żadnych operacji na danych. W Chroma stworzenie klienta wygląda następująco:
```python
chroma_client = chromadb.Client()
```

#### 4.3. Utworzenie kolekcji

Po utworzeniu klienta kolejnym krokiem jest stworzenie kolekcji, czyli miejsca, w którym będą przechowywane dane wektorowe oraz powiązane z nimi dokumenty i metadane.
W bazie Chroma kolekcję można porównać do tabeli w relacyjnych bazach danych. Tak jak tabela przechowuje rekordy w wierszach i kolumnach, tak kolekcja przechowuje dokumenty, ich identyfikatory oraz odpowiadające im osadzenia wektorowe.
Utworzenie kolekcji jest konieczne, ponieważ wszystkie dalsze operacje, takie jak dodawanie dokumentów, wyszukiwanie podobnych treści czy usuwanie danych, wykonywane są właśnie na konkretnej kolekcji.

```python
collection = chroma_client.create_collection(name="my_collection")
```

#### 4.4. Dodanie dokumentu do kolekcji
Dokumenty stanowią właściwe dane, na których później wykonywane będzie wyszukiwanie semantyczne. Dodawanie dokumentów odbywa się za pomocą metody add().

Użytkownik musi znać i wykorzystywać parametry takie jak documents czy ids. Parametr documents zawiera listę tekstów, które mają zostać zapisane w kolekcji. Każdy element listy reprezentuje jeden dokument. 

Parametr ids zawiera unikalny identyfikator dla każdego utworzonego dokumentu. Działa on w sposób podobny do klucza głównego w relacyjnych bazach danych. Poniżej przedstawiono przykład dodania dokumentu.

```python
collection.add(
    ids=["id4"],
    documents=[
        "Ucze sie wektorowych baz danych"
    ]
)
```

#### 4.5. Wyszukiwanie dokumentów w kolekcji
W przeciwieństwie do klasycznych baz danych, gdzie wyszukiwanie opiera się głównie na dokładnym dopasowaniu tekstu, Chroma wykorzystuje wyszukiwanie semantyczne, czyli analizę znaczenia zapytania. Odbywa się to przy pomocy metody query().

Tak samo jak w przypadku dodawania dokumentów trzeba znać istotne parametry takie jak query_texts czy n_results. Query_texts zawiera tekst lub listę tekstów, które mają zostać wyszukane. Chroma umożliwia wyszukiwanie kilku zapytań jednocześnie.

n_results natomiast służy do określania liczby wyników, które mają zostać zwrócone (są to najbardziej podobne dokumenty do naszego tekstu/tekstów).

```python
results = collection.query(
    query_texts=["Czego się uczysz"],
    n_results=1
)
print(results)
```

#### 4.6. Usuwanie danych w Chroma
Podczas pracy z bazą danych często zachodzi potrzeba usunięcia nieaktualnych lub błędnie dodanych danych. W środowisku Chroma możliwe jest usuwanie danych na dwóch poziomach:
* pojedyńczych rekordów,
* całej kolekcji
Usunięcie pojedyńczego rekordu to usunięcie dokumentu o konkretnym identyfikatorze:

```python
collection.delete(ids["id1"])
```

Jeżeli chcemy usunąć wszystkie dokumenty i strukturę kolekcji używamy:

```python
chroma_client.delete_collection(name="my-collection")
```

Analogiczną operacją wykorzystywaną w klasycznych bazach danych jest DROP TABLE.
