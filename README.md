# Fast Text Censor

A blazing-fast multi-language text censoring library for Node.js, powered by Rust and the Aho-Corasick algorithm via [NAPI-RS](https://napi.rs/).

## Features

- ⚡ **Extremely Fast**: Leverages the Aho-Corasick string searching algorithm written in Rust to handle high-throughput text censorship efficiently.
- 🌍 **Multi-Language Support**: Comes bundled with badword dictionaries out of the box (English, Spanish, Indonesian, etc.) inside the `list/` directory.
- 💡 **Smart Leetspeak Detection**: Automatically normalizes tricky inputs and detects repetitive characters to prevent simple workarounds (e.g., bypassing `ass` using `@$$` or `aassss`).
- 🛠 **Customizable**: Supply your own dictionary or add words to the list dynamically at runtime.

## Installation

```bash
npm install fast-text-censor
```

*(Note: Since this library uses native Rust bindings, pre-built binaries will be downloaded during installation for your platform)*

## Usage

### 1. Using the Built-In Dictionaries

You can quickly get started by importing the pre-bundled dictionary (`allWords`) which contains thousands of terms across multiple languages.

```javascript
const { TextCensor, allWords } = require('fast-text-censor');

// Initialize the censor with the default dictionary
const censor = new TextCensor(allWords);

const text = "This is a stupid test with some badwords and ugly things. Dasar anjing kau! Puta madre!";
console.log("Original:", text);

const cleaned = censor.clean(text);
console.log("Cleaned:", cleaned);
```

### 2. Custom Dictionaries

If you only want to censor specific words or load your own custom arrays, you can pass an array of strings into the constructor instead:

```javascript
const { TextCensor } = require('fast-text-censor');

// Initialize with a custom dictionary
const myCensor = new TextCensor(['heck', 'darn', 'shoot']);

console.log(myCensor.clean("What the heck is this?")); 
// Output: "What the **** is this?"
```

### 3. Adding Words Dynamically

You can add new words to an existing censor instance without recreating it:

```javascript
const { TextCensor } = require('fast-text-censor');
const censor = new TextCensor(['badword1']);

censor.addWords(['badword2', 'badword3']);

console.log(censor.clean("I said badword1 and badword2!"));
// Output: "I said ******** and ********!"
```

## API Reference

### `new TextCensor(words: string[])`
Initializes a new `TextCensor` instance.
- **`words`**: An array of strings representing the bad words you want to censor.

### `censor.clean(text: string): string`
Scans the text and replaces any detected bad words with asterisks (`*`). Preserves the original casing and length of the words.

### `censor.addWords(words: string[]): void`
Dynamically adds new words to the existing badword dictionary and automatically rebuilds the search state.

## Contributing

You can add new languages or words by adding `.txt` files to the `list/` directory. Each word should be on a new line. The `allWords` export will automatically pick up any `.txt` files added to that folder.

## License

MIT
