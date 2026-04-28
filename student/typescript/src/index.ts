import { ChromaClient, Collection } from "chromadb";

// 1. Inicjalizacja klienta
async function initClient() {
    return new ChromaClient();
}

// 2. Utworzenie lub pobranie kolekcji
async function getCollection(client: ChromaClient, name: string) {
    return await client.getOrCreateCollection({
        name: name,
    });
}

// 3. Dodanie dokumentów
async function addDocs(collection: Collection, ids: string[], documents: string[]) {
    await collection.add({
        ids: ids,
        documents: documents,
    });
}

// 4. Wyszukiwanie semantyczne
async function searchDocs(collection: Collection, query: string, n: number = 2) {
    return await collection.query({
        queryTexts: [query],
        nResults: n,
    });
}

// 5. Usuwanie rekordów
async function removeDocs(collection: Collection, ids: string[]) {
    await collection.delete({
        ids: ids,
    });
}

/*
Przykład użycia
 */
async function main() {
    try {
        console.log("--- Rozpoczynam pracę z ChromaDB ---");

        const client = await initClient();

        const collectionName = "moja_kolekcja_testowa";
        const collection = await getCollection(client, collectionName);
        console.log(`Połączono z kolekcją: ${collectionName}`);

        console.log("Dodawanie dokumentów...");
        await addDocs(
            collection,
            ["id1", "id2"],
            ["To jest dokument o owocach cytrusowych", "To jest dokument o szybkich samochodach"]
        );

        console.log("Wyszukiwanie zapytania: 'Coś do jedzenia'...");
        const results = await searchDocs(collection, "Coś do jedzenia", 1);
        console.log("Wynik wyszukiwania:", results.documents);

        console.log("Usuwanie dokumentu id1...");
        await removeDocs(collection, ["id1"]);

        console.log("--- Wszystkie operacje zakończone pomyślnie ---");

    } catch (error) {
        console.error("Wystąpił błąd:", error);
    }
}

main();
