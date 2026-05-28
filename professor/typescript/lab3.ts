import * as fs from 'fs';
import * as readline from 'readline';
import { ChromaClient } from 'chromadb';
import { FlagEmbedding, EmbeddingModel } from 'fastembed';
import ollama from 'ollama';


const client = new ChromaClient();

let embedder: FlagEmbedding;

function chunkText(text: string): string[] {
    const chunks: string[] = [];
    let currentChunk: string[] = [];

    for (const line of text.split("\n")) {
        if (line.startsWith("##")) {
            if (currentChunk.length > 0) {
                chunks.push(currentChunk.join("\n"));
                currentChunk = [];
            }
        }
        currentChunk.push(line);
    }

    if (currentChunk.length > 0) {
        chunks.push(currentChunk.join("\n"));
    }

    return chunks.map(chunk => chunk.trim()).filter(chunk => chunk.trim().length > 0);
}

async function buildIndex(text: string) {
    const chunks = chunkText(text);

    embedder = await FlagEmbedding.init({ model: EmbeddingModel.BGESmallENV15 });

    const collectionName = "hr_policy";
    try {
        await client.deleteCollection({ name: collectionName });
    } catch (e) {
    }
    const collection = await client.createCollection({ name: collectionName, embeddingFunction: null });

    for (let i = 0; i < chunks.length; i++) {
        const chunk = chunks[i];

        const generator = embedder.embed([chunk]);
        let vector: number[] = [];
        for await (const batch of generator) {
            vector = Array.from(batch[0]);
        }

        await collection.add({
            documents: [chunk],
            embeddings: [vector],
            ids: [`doc_${i}`]
        });
    }

    console.log(`Dodano ${chunks.length} fragmentów do bazy.`);
}

async function retrieve(query: string, k: number = 3): Promise<string[]> {
    const queryVector = Array.from(await embedder.queryEmbed(query));

    const collectionName = "hr_policy";
    const collection = await client.getCollection({ name: collectionName, embeddingFunction: null });

    const results = await collection.query({
        queryEmbeddings: [queryVector],
        nResults: k
    });

    return (results.documents[0] as string[]) || [];
}

async function askHrBot(question: string): Promise<string> {
    const documents = await retrieve(question);
    const context = documents.join("\n\n");

    const prompt = `Jesteś asystentem HR.

Odpowiadaj tylko na podstawie podanego kontekstu.

Jeżeli nie ma informacji w kontekście, napisz:
"Brak informacji w dokumentacji."

Kontekst:
${context}

Pytanie:
${question}`;

    try {
        const response = await ollama.chat({
            model: "llama3",
            messages: [
                {
                    role: "user",
                    content: prompt
                }
            ]
        });

        return response.message.content;
    } catch (error: any) {
        console.error("Błąd podczas komunikacji z Ollama:", error.message);
        return `Przepraszam, wystąpił problem podczas komunikacji z Ollama (upewnij się, że Ollama jest uruchomiona).
Udało się jednak pomyślnie pobrać kontekst z bazy wektorowej.
Pobrany kontekst z bazy danych:\n\n${context}`;
    }
}

async function chat() {
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout
    });

    console.log("\nHR Assistant");
    console.log("Wpisz 'exit' aby zakończyć.\n");

    const askQuestion = () => {
        rl.question("Ty: ", async (question) => {
            if (question.toLowerCase() === 'exit') {
                rl.close();
                return;
            }

            const answer = await askHrBot(question);
            console.log(`\nBot: ${answer}\n`);

            askQuestion();
        });
    };

    askQuestion();
}

async function main() {
    const fileName = "dane.md";
    console.log("Wczytywanie pliku:", fileName);

    if (!fs.existsSync(fileName)) {
        console.error(`Błąd: Nie znaleziono pliku ${fileName}`);
        return;
    }

    const text = fs.readFileSync(fileName, 'utf-8');
    await buildIndex(text);
    await chat();
}

main().catch(console.error);
