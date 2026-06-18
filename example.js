const { TextCensor, allWords } = require('./lib.js');

const censor = new TextCensor(allWords);

const text = "This is a stupid test with some badwords and ugly things. Dasar anjing kau! Puta madre!";
console.log("Original text:", text);
const cleaned = censor.clean(text);
console.log("Cleaned text:", cleaned);

censor.addWords(['heck']);
const moreCleaned = censor.clean("What the heck is this badword?");
console.log("More cleaned text:", moreCleaned);

const customCleaned1 = censor.cleanCustom("Dasar anjing kau!", { customChar: "$", keepFirstLast: false });
console.log("Custom cleaned ($):", customCleaned1);

const customCleaned2 = censor.cleanCustom("Dasar anjing kau!", { keepFirstLast: true });
console.log("Custom cleaned (keepFirstLast):", customCleaned2);

const customCleaned3 = censor.cleanCustom("Dasar anjing kau!", { customChar: "@", keepFirstLast: true });
console.log("Custom cleaned (@, keepFirstLast):", customCleaned3);

censor.addWords(['asu']);
const scunthorpeText = "Beli kasur baru dasar asu!";
console.log("\nScunthorpe Problem Test:");
console.log("Original:", scunthorpeText);
console.log("Without matchWholeWord:", censor.cleanCustom(scunthorpeText, { keepFirstLast: false }));
console.log("With matchWholeWord:", censor.cleanCustom(scunthorpeText, { matchWholeWord: true }));

console.log("\nLeetspeak & Normalization Test:");
console.log("Original: Dasar b4bi kau, dasar 4nj1n9, serta a.s.u!");
console.log("Cleaned:", censor.clean("Dasar b4bi kau, dasar 4nj1n9, serta a.s.u!"));

console.log("\nWhitelist (ignoreList) Test:");
console.log("Original: Dasar anjing kau, tapi anjing laut itu lucu!");
console.log("Without ignoreList:", censor.cleanCustom("Dasar anjing kau, tapi anjing laut itu lucu!", {}));
console.log("With ignoreList (ignore 'anjing'):", censor.cleanCustom("Dasar anjing kau, tapi anjing laut itu lucu!", { ignoreList: ["anjing"] }));

console.log("\nMatch Details (getMatches) Test:");
const matchText = "Test b4bi dengan emoji 👍 dan a.s.u";
console.log("Original:", matchText);
const matches = censor.getMatches(matchText);
console.log("Matches:", matches);
console.log("Verifying slice 1:", matchText.slice(matches[0].start, matches[0].end));
console.log("Verifying slice 2:", matchText.slice(matches[1].start, matches[1].end));

async function runAsyncTests() {
    console.log("\n--- Async Tests ---");
    const longText = "This is a stupid test. ".repeat(1000);

    console.time("Async Clean");
    const resultAsync = await censor.cleanAsync(longText);
    console.timeEnd("Async Clean");
    console.log("Async cleaned length:", resultAsync.length);

    console.time("Async Custom Clean");
    const customResultAsync = await censor.cleanCustomAsync(longText, { keepFirstLast: true });
    console.timeEnd("Async Custom Clean");
    console.log("Async custom cleaned length:", customResultAsync.length);
}

runAsyncTests().catch(console.error);

// Trigger CI
