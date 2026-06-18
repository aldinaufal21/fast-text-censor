# Fast Text Censor

A blazing-fast multi-language text censoring library for Node.js, powered by Rust and the Aho-Corasick algorithm via [NAPI-RS](https://napi.rs/).

## Features

- ⚡ **Extremely Fast**: Leverages the Aho-Corasick string searching algorithm written in Rust to handle high-throughput text censorship efficiently.
- 🌍 **Multi-Language Support**: Comes bundled with badword dictionaries out of the box (English, Spanish, Indonesian, etc.) inside the `list/` directory.
- 💡 **Smart Leetspeak & Bypass Detection**: Automatically normalizes tricky inputs (`b4bi`, `@.s.u`) and detects repetitive characters (`anjiiing`) to prevent simple workarounds.
- 🛡️ **Scunthorpe Problem Prevention**: Optional strict word-boundary matching to prevent censoring innocent words (e.g. `kasur` won't be censored for `asu`).
- 🔍 **Match Details (Analytics)**: Exposes the exact coordinates (`start`/`end`) and matched words for logging or user banning algorithms.
- 🚀 **Asynchronous / Non-Blocking**: Fully supports offloading multi-megabyte string processing to a background thread pool, keeping your Node.js event loop completely free.
- 🛠 **Customizable**: Supply your own dictionary, customize the replacement character, or preserve first/last characters.

## Installation

```bash
npm install fast-text-censor
```

*(Note: Since this library uses native Rust bindings, pre-built binaries will be downloaded during installation for your platform)*

## Usage

### 1. Basic Usage

```javascript
const { TextCensor, allWords } = require('fast-text-censor');

// Initialize the censor with the default dictionary
const censor = new TextCensor(allWords);

console.log(censor.clean("Dasar anjing kau!"));
// Output: "Dasar ****** kau!"
```

### 2. Custom Replacements & Formatting

You can customize how the censored words appear using `cleanCustom()`.

```javascript
// Replace with a specific character
console.log(censor.cleanCustom("Dasar anjing kau!", { customChar: "#" }));
// Output: "Dasar ###### kau!"

// Preserve the first and last letters for context
console.log(censor.cleanCustom("Dasar anjing kau!", { keepFirstLast: true }));
// Output: "Dasar a****g kau!"
```

### 3. Fixing the Scunthorpe Problem

Prevent the censor from triggering on innocent words that happen to contain a bad word as a substring.

```javascript
const censor = new TextCensor(["asu"]);

const text = "Beli kasur baru dasar asu!";

// Without matchWholeWord
console.log(censor.cleanCustom(text, {}));
// Output: "Beli k***r baru d**ar ***!"

// With matchWholeWord (Only independent words are censored)
console.log(censor.cleanCustom(text, { matchWholeWord: true }));
// Output: "Beli kasur baru dasar ***!"
```

### 4. Whitelisting (ignoreList)

If you want to explicitly ignore certain bad words in a specific context:

```javascript
const text = "Dasar anjing kau, tapi anjing laut itu lucu!";

console.log(censor.cleanCustom(text, { ignoreList: ["anjing"] }));
// Output: "Dasar anjing kau, tapi anjing laut itu lucu!"
```

### 5. Getting Match Details (Analytics)

If you need to know exactly *what* was detected and *where* it is located (e.g., to highlight it in the UI or keep analytics). The returned indices are UTF-16 code units, meaning they perfectly align with JavaScript's `String.prototype.slice()`.

```javascript
const text = "Test b4bi dengan emoji 👍 dan a.s.u";
const matches = censor.getMatches(text);

console.log(matches);
/* Output:
[
  { word: 'babi', matchText: 'b4bi', start: 5, end: 9 },
  { word: 'asu', matchText: 'a.s.u', start: 30, end: 35 }
]
*/

console.log(text.slice(matches[0].start, matches[0].end)); // "b4bi"
```

### 6. Asynchronous / Non-Blocking Execution

For extremely long texts (like gigabyte-sized log files or huge document bodies), use the Async API to process them in a background thread without freezing your Node.js server.

```javascript
async function processLargeText() {
  const hugeString = "Badword ".repeat(100000);

  // Standard clean
  const result = await censor.cleanAsync(hugeString);
  
  // Custom clean
  const customResult = await censor.cleanCustomAsync(hugeString, {
    keepFirstLast: true,
    matchWholeWord: true
  });
}
```

## API Reference

### `new TextCensor(words: string[])`
Initializes a new `TextCensor` instance.

### `censor.clean(text: string): string`
Scans the text and replaces detected bad words with asterisks (`*`).

### `censor.cleanCustom(text: string, options: CensorOptions): string`
Censors text based on the provided options:
- `customChar?: string` - The character to replace bad words with.
- `keepFirstLast?: boolean` - Retains the first and last characters of the censored word.
- `matchWholeWord?: boolean` - Enforces word-boundary checks (Scunthorpe prevention).
- `ignoreList?: string[]` - Words to explicitly skip censoring.

### `censor.cleanAsync(text: string): Promise<string>`
Asynchronous equivalent of `clean()`.

### `censor.cleanCustomAsync(text: string, options: CensorOptions): Promise<string>`
Asynchronous equivalent of `cleanCustom()`.

### `censor.getMatches(text: string, options?: CensorOptions): MatchResult[]`
Returns an array of objects detailing the detected bad words, their matched substrings, and their exact start/end locations.

### `censor.addWords(words: string[]): void`
Dynamically adds new words to the existing badword dictionary and automatically rebuilds the search state.

## Contributing

You can add new languages or words by adding `.txt` files to the `list/` directory. Each word should be on a new line. The `allWords` export will automatically pick up any `.txt` files added to that folder.

## License

MIT
