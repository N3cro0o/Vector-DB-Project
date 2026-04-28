## Laboratorium 1 - TypeScript

W nowoczesnej inżynierii oprogramowania operacje na bazach danych są nieblokującymi operacjami wejścia/wyjścia (I/O). Oznacza to, że silnik TypeScript wysyła zapytanie i kontynuuje wykonywanie innych zadań, czekając na odpowiedź z sieci.
* **Asynchroniczność i await:** Każda interakcja z Chroma DB zwraca obiekt Promise. Słowo kluczowe await jest krytyczne, instruuje ono środowisko wykonawcze, aby wstrzymało dalszą realizację danej funkcji do momentu otrzymania danych z serwera, nie blokując przy tym głównego wątku aplikacji.
* **Szybkie prototypowanie z `ts-node`:** Aby uniknąć żmudnego procesu ręcznej transpilacji plików .ts do .js podczas laboratorium, wykorzystujemy `ts-node`. Narzędzie to pozwala na bezpośrednie wykonywanie kodu TypeScript.
Zaleca się używanie `npx ts-node nazwa_pliku.ts` pozwala to na uruchomienie narzędzia bez konieczności jego globalnej instalacji, co jest dobrą praktyką w zarządzaniu zależnościami. Upewnij się, że w głównym katalogu projektu znajduje się plik konfiguracji `tsconfig.json`.

### 1 Lokalna instalacja Chroma
Architektura rozwiązania opiera się na modelu klient-serwer. Biblioteka TypeScript pełni rolę klienta, który komunikuje się z backendem Chroma za pośrednictwem protokołu HTTP. Oznacza to, że przed uruchomieniem kodu aplikacji musi działać aktywny proces serwera.

#### Konfiguracja serwera (Backend)
Serwer Chroma można uruchomić na dwa sposoby. Kluczowe jest zapewnienie trwałości danych poprzez wskazanie ścieżki zapisu.

* **Opcja 1:** Użycie Python/Pip Jeśli posiadasz środowisko Python zainstaluj chromadb: 
    `pip install chromadb`

    a następnie uruchom serwer komendą:  `chroma run --path ./lab1`

* **Opcja 2:** Konteneryzacja (Docker) Dla pełnej izolacji środowiska można wykorzystać oficjalny obraz:

    `docker run -p 8000:8000 chromadb/chroma`

* **Opcja 3:** Samodzielny instalator CLI (Standalone Installer) – Możesz pobrać binarną wersję narzędzia bezpośrednio za pomocą 

    cURL (macOS/Linux): `curl -sSL https://raw.githubusercontent.com/chroma-core/chroma/main/rust/cli/install/install.sh | bash`

    lub 

    PowerShell (Windows): `iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/chroma-core/chroma/main/rust/cli/install/install.ps1'))`

    Po zakończeniu instalacji włącz serwer komendą: 
 
    `chroma run --path ./lab1`

 Uwaga: Parametr `--path` definiuje folder, w którym Chroma będzie trwale przechowywać Twoje indeksy. Bez niego dane mogą zostać utracone po restarcie serwera.


#### Konfiguracja aplikacji (Client)
Po uruchomieniu backendu, w katalogu swojego projektu TypeScript zainstaluj niezbędną bibliotekę kliencką:

`npm install chromadb`

(możesz użyć również `pnpm`, `bun` lub `yarn`)

#### Pakiety funkcji osadzających (Embedding Functions)
W ekosystemie JS/TS funkcje odpowiedzialne za zamianę tekstu na wektory znajdują się w dedykowanej przestrzeni nazw @chroma-core/*
* Na potrzeby tego laboratorium wykorzystujemy pakiet dla domyślnego modelu: 

    `npm install @chroma-core/default-embed`

* Warto wiedzieć, że istnieją również pakiety dla zewnętrznych dostawców, takich jak `@chroma-core/openai` czy `@chroma-core/cohere`


### 2 Utworzenie klienta

Pierwszym krokiem podczas pracy z bazą danych Chroma jest utworzenie klienta, czyli obiektu odpowiedzialnego za nawiązanie połączenia z bazą oraz wykonywanie na niej operacji. Klient pełni rolę interfejsu komunikacyjnego pomiędzy programem użytkownika a bazą danych.

W zależności od konfiguracji projektu (ESM lub CJS) import wygląda następująco:

**TypeScript ESM (Zalecane):**
```typescript
import { ChromaClient } from "chromadb";
const client = new ChromaClient();
```

**TypeScript CJS:**
```typescript
const { ChromaClient } = require("chromadb");
const client = new ChromaClient();
```
Domyślnie klient łączy się z adresem `http://localhost:8000`.

### 3 Utworzenie kolekcji

Po utworzeniu klienta kolejnym krokiem jest stworzenie kolekcji, czyli miejsca, w którym będą przechowywane dane wektorowe oraz powiązane z nimi dokumenty i metadane.
W bazie Chroma kolekcję można porównać do tabeli w relacyjnych bazach danych. Tak jak tabela przechowuje rekordy w wierszach i kolumnach, tak kolekcja przechowuje dokumenty, ich identyfikatory oraz odpowiadające im osadzenia wektorowe.
Utworzenie kolekcji jest konieczne, ponieważ wszystkie dalsze operacje, takie jak dodawanie dokumentów, wyszukiwanie podobnych treści czy usuwanie danych, wykonywane są właśnie na konkretnej kolekcji.

```typescript
const collection = await client.createCollection({
    name: "my_collection"
});
```
Ponieważ operacje na bazie są asynchroniczne, należy używać słowa kluczowego `await`.

### 4 Dodanie dokumentu do kolekcji
Dokumenty stanowią właściwe dane, na których później wykonywane będzie wyszukiwanie semantyczne. Dodawanie dokumentów odbywa się za pomocą metody `add()`.

Użytkownik musi znać i wykorzystywać parametry takie jak documents czy ids. Parametr documents zawiera listę tekstów, które mają zostać zapisane w kolekcji. Każdy element listy reprezentuje jeden dokument. 

Parametr ids zawiera unikalny identyfikator dla każdego utworzonego dokumentu. Działa on w sposób podobny do klucza głównego w relacyjnych bazach danych. Poniżej przedstawiono przykład dodania dokumentu.

```typescript
await collection.add({
    ids: ["id1", "id2"],
    documents: [
        "This is a document about pineapple",
        "This is a document about oranges"
    ]
});
```
### 5 Wyszukiwanie dokumentów w kolekcji
W przeciwieństwie do klasycznych baz danych, gdzie wyszukiwanie opiera się głównie na dokładnym dopasowaniu tekstu, Chroma wykorzystuje wyszukiwanie semantyczne, czyli analizę znaczenia zapytania. Odbywa się to przy pomocy metody `query()`.

Tak samo jak w przypadku dodawania dokumentów trzeba znać istotne parametry takie jak `queryTexts` czy `nResults`, `queryTexts` zawiera tekst lub listę tekstów, które mają zostać wyszukane. Chroma umożliwia wyszukiwanie kilku zapytań jednocześnie.

`nResults` natomiast służy do określania liczby wyników, które mają zostać zwrócone (są to najbardziej podobne dokumenty do naszego tekstu/tekstów).

```typescript
const results = await collection.query({
    queryTexts: ["This is a query about fruit"],
    nResults: 2
});
console.log(results);
```

### 6 Usuwanie danych w Chroma
Podczas pracy z bazą danych często zachodzi potrzeba usunięcia nieaktualnych lub błędnie dodanych danych. W środowisku Chroma możliwe jest usuwanie danych na dwóch poziomach:
* pojedyńczych rekordów,
* całej kolekcji
Usunięcie pojedyńczego rekordu to usunięcie dokumentu o konkretnym identyfikatorze:

```typescript
await collection.delete({
    ids: ["id1"]
});
```

Jeżeli chcemy usunąć wszystkie dokumenty i strukturę kolekcji używamy:

```typescript
await client.deleteCollection({
    name: "my_collection"
});
```

Analogiczną operacją wykorzystywaną w klasycznych bazach danych jest DROP TABLE.

### 7 Źródła
[Chroma Official Docs: Getting Started](https://docs.trychroma.com/docs/overview/getting-started)

[Chroma Cookbook: Installation Guide](https://cookbook.chromadb.dev/core/install/)
