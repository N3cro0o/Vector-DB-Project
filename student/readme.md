## Laboratorium - Wstęp

### Czym jest wektorowa baza danych?

Wektorowa baza danych zwana też magazynem wektorowym lub wektorową wyszukiwarką to baza danych, która przechowuje i pobiera reprezentację danych w przestrzeni wektorowej. Wektorowe bazy danych zazwyczaj wykorzystują algorytmy przybliżonego najbliższego sąsiedztwa, dzięki czemu użytkownicy mogą wyszukiwać rekordy semantycznie podobne, czyli takie, które mają podobne znaczenie lub kontekst. Tradycyjne bazy danych wyszukują rekordy głównie na podstawie dokładnego dopasowania. Przykłady zastosowań wektorowych baz danych obejmują wyszukiwanie podobieństw, wyszukiwanie semantyczne, wyszukiwanie multimodalne, silniki rekomendacji, wykrywanie obiektów oraz generowanie wspomagane wyszukiwaniem (RAG). Osadzenia wektorowe są matematycznymi reprezentacjami danych w przestrzeni wielowymiarowej. Ich kombinacja odzwierciedla cechy i relacje między obiektami. W tej przestrzeni każdy wymiar odpowiada jednej cesze danych, a liczba wymiarów waha się od kilkuset do kilkudziesięciu tysięcy w zależności od złożoności reprezentowanych danych. Każdy element danych jest reprezentowany przez jeden wektor w tej przestrzeni. Każdy element danych – słowo, fraza, dokument, obraz czy plik audio – może być przekształcony w wektor.

### Dostępne wektorowe bazy danych

#### 2.1. Chroma
Chroma jest to baza wektorowa open-source zaprojektowana do integracji z aplikacjami opartymi o modele językowe. Główne założenia tego rozwiązania to lokalne postawienie bazy danych oraz prostota implementacji i pracy. Praca z tym środowiskiem oparta jest o język Python natomiast dostępne są alternatywne języki takie jak Rust czy TypeScript. System wspiera automatyczne zarządzanie kolekcjami i integruje je bezpośrednio z dodatkowym frameworkiem. Posiada licencje Apache License 2.0.

Dokumentacja:
https://www.trychroma.com/

#### 2.2. Pinecone
Pinecone to zarządzana, natywna dla chmury baza wektorowa zaprojektowana do przechowywania, indeksowania i szybkiego wyszukiwania wielowymiarowych wektorów. Wyróżnia się tym, że usuwa konieczność ręcznego zarządzania infrastrukturą i jest automatycznie skalowana do potrzeb aplikacji. Łatwo integruje się z Pythonem dzięki SDK pinecone-client. Jest to komercyjna platforma SaaS (Software-as-a-Service).

Dokumentacja:
https://docs.pinecone.io/guides/get-started/overview

#### 2.3. Milvus
Milvus to otwartoźródłowa, wysoce skalowalna i wydajna wektorowa baza danych. Stworzona z myślą o zarządzaniu ogromnymi zbiorami nieustrukturyzowanych danych przekształconych na wektory. Charakteryzuje się wyjątkową elastycznością dzięki trzem trybom wdrożenia oraz skalowalnością. Dzięki optymalizacji sprzętowej, zastosowaniu zaawansowanych algorytmów wyszukiwania, silnika napisanego w C++ oraz architektury zorientowanej kolumnowo imponuje swoją szybkością działania. Posiada zarówno SDK do Pythona (PyMilvus) jak i do Javy (Milvus Java). Milvus działa w oparciu o licencje Apache License 2.0.

Dokumentacja:
https://milvus.io/docs

#### 2.4. Qdrant
Napisana od podstaw w języku programowania Rust, w pełni otwartoźródłowa wektorowa baza danych, która kładzie ogromny nacisk na szybkość zapytań i wydajne zarządzanie pamięcią maszynową. Rozwiązanie to słynie na rynku z zaawansowanego mechanizmu filtrowania wewnątrz grafu, który aplikuje warunki dla metadanych już w trakcie nawigacji po strukturze Hierarchical Navigable Small World (HNSW). Baza wspiera instalację lokalną za pomocą Dockera i integruje się z lekkimi bibliotekami do generowania osadzeń. Posiada także SDK do Pythona (qdrant-client). Qdrant oparty jest o licencje Apache License 2.0.

Dokumentacja:
https://qdrant.tech/documentation/

#### 2.5. PostreSQL + pgvector
Pgvector jest rozszerzeniem aplikacji PostgreSQL pozwalającym na korzystanie i wyszukiwanie danych wektorowych. Dodatkowo pozwala na zachowanie gotowych rozszerzeń. Rozwiązanie to łączy relacyjną bazę danych z wektorowym rozwiązaniem, co pozwala na wykorzystanie zasad ACID i transakcji. Dodatkowym atutem jest możliwość rozszerzenia standardowych tabel o kolumny rozszerzeń wektorowych. Mamy tutaj do czynienia z licencją PostgreSQL License.

Dokumentacja:
https://github.com/pgvector/pgvector

#### 2.6. LanceDB
LanceDB jest bazą wektorową typu open‑source, która przechowuje dane w formie kolumnowej. Daje możliwość łączenia wyszukiwania wektorowego z filtrowaniem metadanych lub wyszukiwaniem słów kluczowych (wyszukiwanie hybrydowe). Wspiera zapytania SQL. LanceDB posiada licencję Apache License 2.0.
Dokumentacja:
https://docs.lancedb.com/index

#### 2.7. Vectra
Vectra to oparta na plikach, działająca w pamięci wektorowa baza danych dla Node.js. Działa podobnie jak Pinecone czy Qdrant: każdy indeks to folder na dysku zawierający plik JSON z wektorami i polami metadanych. Wszystkie pozostałe metadane są przechowywane dla poszczególnych elementów jako oddzielne pliki JSON. Ponieważ cały indeks jest ładowany do pamięci, wynik wyszukiwania jest zwracany w bardzo krótkim czasie (średnio <1 ms). Głównie wykorzystywanymi językami jest JavaScript i Node.js.

#### 2.8. Azure AI Search
To usługa w chmurze Microsoft Azure, która umożliwia wyszukiwanie wektorowe oraz hybrydowe, które łączy tradycyjne wyszukiwanie słów kluczowych z semantycznym dopasowaniem wektorów co umożliwia tworzenie bardziej zaawansowanych zapytań. Można z niej korzystać zarówno przy wykorzystaniu C# jak i Pythona (biblioteka azure-search-documents). Azure AI Search nie korzysta z tradycyjnej licencji oprogramowania. Opiera się na modelu subskrypcyjnym (płatność za użycie - pay-as-you-go) w ramach platformy Microsoft Azure.

Dokumentacja:
https://learn.microsoft.com/pl-pl/azure/search/
