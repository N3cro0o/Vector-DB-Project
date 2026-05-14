import { ChromaClient } from "chromadb";
import { FlagEmbedding, EmbeddingModel } from "fastembed";

async function main() {
    console.log("Ładowanie modelu...");
    const model = await FlagEmbedding.init({ model: EmbeddingModel.BGESmallENV15 });

    // Punkt 1: Inicjalizacja klienta i tworzenie kolekcji
    const client = new ChromaClient();

    try {
        await client.deleteCollection({ name: "countries" });
    } catch (e) {
    }
    const collection = await client.createCollection({ name: "countries" });

    // Punkt 2: Dokumenty
    const documents = [
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
    ];

    // Punkt 3: Metadane
    const metadatas = [
        { continent: "Europe", capital: "Warsaw", currency: "PLN" },
        { continent: "Europe", capital: "Berlin", currency: "EUR" },
        { continent: "Europe", capital: "Paris", currency: "EUR" },
        { continent: "Europe", capital: "Rome", currency: "EUR" },
        { continent: "Europe", capital: "Madrid", currency: "EUR" },
        { continent: "North America", capital: "Washington D.C.", currency: "USD" },
        { continent: "North America", capital: "Ottawa", currency: "CAD" },
        { continent: "Asia", capital: "Tokyo", currency: "JPY" },
        { continent: "Asia", capital: "Beijing", currency: "CNY" },
        { continent: "South America", capital: "Brasília", currency: "BRL" }
    ];

    const ids = documents.map((_, i) => i.toString());


    // Punkt 4: Generowanie embeddingów
    console.log("Generowanie embeddingów dla dokumentów...");
    const embeddingsGenerator = model.embed(documents);
    const embeddings: number[][] = [];
    for await (const batch of embeddingsGenerator) {
        embeddings.push(...batch.map(e => Array.from(e)));
    }

    // Punkt 5: Zapis do bazy ChromaDB
    console.log("Zapis do bazy ChromaDB...");
    await collection.add({
        ids: ids,
        embeddings: embeddings,
        metadatas: metadatas,
        documents: documents
    });

    async function getQueryEmbedding(text: string): Promise<number[]> {
        return Array.from(await model.queryEmbed(text));
    }


    // Punkt 6: Wyszukiwanie semantyczne państwa o podobnym opisie
    let query = "country with strong economy in Europe";
    let queryEmbedding = await getQueryEmbedding(query);

    let results = await collection.query({
        queryEmbeddings: [queryEmbedding],
        nResults: 1
    });

    console.log("\n=== wyszukiwanie semantyczne ===");
    results.documents[0]?.forEach(doc => console.log("-", doc));

    // Punkt 7: Filtrowanie przy użyciu metadanych
    query = "country";
    queryEmbedding = await getQueryEmbedding(query);

    results = await collection.query({
        queryEmbeddings: [queryEmbedding],
        nResults: 5,
        where: { continent: "Europe" } // Filtr: Szukaj tylko w Europie
    });

    console.log("\n=== państwa w Europie (filtrowanie po kontynencie) ===");
    results.documents[0]?.forEach(doc => console.log("-", doc));

    // --- Przykład filtrowania po konkretnej wartości (stolica) ---
    query = "capital cities";
    queryEmbedding = await getQueryEmbedding(query);

    results = await collection.query({
        queryEmbeddings: [queryEmbedding],
        nResults: 5,
        where: { capital: "Tokyo" }
    });

    console.log("\n=== filtrowanie po stolicy (Tokyo) ===");
    results.documents[0]?.forEach(doc => console.log("-", doc));

    // --- Przykład pokazujący nResults: 2 (Top 2 podobne państwa) ---
    query = "big country with many people in Asia";
    queryEmbedding = await getQueryEmbedding(query);

    results = await collection.query({
        queryEmbeddings: [queryEmbedding],
        nResults: 2 // Zadanie 5.4 wymaga znalezienia 2 podobnych państw
    });

    console.log("\n=== Top 2 podobne państwa ===");
    results.documents[0]?.forEach(doc => console.log("-", doc));
}

main().catch(console.error);