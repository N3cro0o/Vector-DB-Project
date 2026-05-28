import * as fs from 'fs';
import * as readline from 'readline';
import { ChromaClient } from 'chromadb';
import { FlagEmbedding } from 'fastembed';
import ollama from 'ollama';

function loadMarkdown(filePath: string): string {
    return fs.readFileSync(filePath, 'utf-8');
}

const client = new ChromaClient();

let embedder: FlagEmbedding;

async function buildIndex(text: string): Promise<void> {
    // TODO: Zaimplementuj podział tekstu na fragmenty, generowanie osadzeń i zapis do bazy ChromaDB
}

async function askHrBot(question: string): Promise<string> {
    // TODO: Zaimplementuj wyszukiwanie w bazie wektorowej i generowanie odpowiedzi przez Ollama
    return "";
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

    const text = loadMarkdown(fileName);

    await buildIndex(text);
    await chat();
}

main().catch(console.error);