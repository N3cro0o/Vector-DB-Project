## Laboratorium 3 - Python

### 6.1. Co to jest RAG?

Technologia Retrieval-Augmented Generation (RAG) łączy możliwość generowania odpowiedzi poprzez duże modele językowe z mechanizmem wyszukiwania informacji. Tradycyjne modele generowania tekstu opierają się na danych, które zostały w nich wytrenowane. RAG rozszerza te możliwości poprzez dodanie etapu wyszukiwania, który pozwala modelowi na dostęp do zewnętrznych źródeł informacji w czasie rzeczywistym. Dzięki temu model może generować odpowiedzi, które są bardziej trafne i aktualne.

RAG działa w dwóch głównych etapach:

* Etap wyszukiwania (retrieval),
* Etap generowania (generation),


RAG znajduje zastosowanie w chatbotach, systemach rekomendacyjnych, narzędziach do analizy dokumentów, a nawet w medycynie do wspomagania decyzji klinicznych. Na przykład chatbot wykorzystujący architekturę RAG może nie tylko odpowiadać na pytania użytkowników na podstawie swojej bazy wiedzy, ale także wyszukiwać najnowsze informacje, aby udzielać bardziej precyzyjnych odpowiedzi.

### 6.2. Chunking dokumentów

Chunking to sposób dzielenia treści na mniejsze, logicznie spójne fragmenty zawierające nagłówki, treść oraz wyróżnione elementy. Każdy taki fragment powinien być zrozumiały w oderwaniu od reszty treści i skupiony wyłącznie wokół jednego pomysłu, pytania czy zagadnienia. 

Modele AI mają ograniczone okno kontekstowe i działają najlepiej, gdy otrzymują precyzyjne, zwięzłe fragmenty informacji — nie całe dokumenty. Trzeba znaleźć balans pomiędzy zbyt dużymi chunkami, które powodują szum, a zbyt małymi, które tracą kontekst danej wypowiedzi.

W architekturze RAG odpowiedni podział na chunki ma kluczowe znaczenie, ponieważ to właśnie te fragmenty są wyszukiwane w bazie wektorowej i przekazywane do modelu językowego jako kontekst odpowiedzi.

### 6.3 Ollama

W laboratorium wykorzystany zostanie lokalny model językowy uruchamiany przy pomocy narzędzia Ollama. Pozwala ono uruchamiać modele LLM lokalnie na komputerze użytkownika bez konieczności korzystania z zewnętrznych API oraz kluczy dostępowych w porównaniu do OpenAI. Jest to darmowa, otwartoźródłowa platforma.

Ollama dodawana jest do systemu RAG, aby zebrać znalezione fragmenty tekstu w etapie wyszukiwania, zrozumieć je i wygenerować naturalną odpowiedź podobną do tej, która wypowiedziałby człowiek. 

1. Do rozpoczęcia pracy z Ollamą należy pobrać serwer ze strony: https://ollama.com/download?utm_source=chatgpt.com
2. Uruchom instalator i wykonaj standardową instalację.
3. Po zakończeniu instalacji uruchom terminal (najlepiej zamknąć Pythona i uruchomić ponownie po instalacji) i sprawdź poprawność instalacji przy pomocy komendy:

```python
ollama --version
```
Jeżeli zostanie wyświetlona wersja programu to oznacza, że wszystko zostało poprawnie zainstalowane. 

Aby zainstalować Ollame w Pythonie należy w terminalu wpisać komendę:

```python
pip install ollama
```

### 6.4 Model llama3

Po zainstalowaniu serwera Ollama należy pobrać model językowy wykorzystywany podczas laboratorium.

W terminalu wykonaj polecenie:
```python
ollama pull llama3
```

Aby uruchomić model i sprawdzić jego działanie należy podać komendę:
```python
ollama run llama3
```

Po utworzeniu modelu w Pythonie będziesz miał możliwość wpisywanie pytań bezpośrednio z terminalu.

### 6.5. Zadanie dla studentów

Celem zadania jest zbudowanie lokalnego systemu QA (Question Answering) typu RAG (Retrieval Augmented Generation), który odpowiada na pytania użytkownika na podstawie dokumentu HR zapisanym w pliku .md. Na podstawie tego dokumentu możesz zbudować pytania do tekstu i sprawdzić czy system działa poprawnie.

Przykład:

**Pytanie** - Ty: Ile dni wcześniej należy zgłosić chęć urlopu wypoczynkowego.

**Odpowiedź** - Bot: Minimum 7 dni wcześniej.

System ma wykorzystywać:

* embeddingi (Fastembed),
* bazę wektorową (ChromaDB),
* model językowy (Ollama – Llama3),
* prosty interfejs CLI,

W pliku pomoc możesz zobaczyć w jaki sposób wczytać plik z dokumentem, jak zbudować interfejs tekstowy i zobaczyć przykład uruchomienia całego programu. ask_hr_bot i build_index musisz zaimplementować gdyż są to funkcje odpowiedzialne odpowiednio za generowanie odpowiedzi przez model językowy oraz zbudowanie indeksu w bazie wektorowej. 