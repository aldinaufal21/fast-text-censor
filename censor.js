const fs = require('fs');
const path = require('path');

// Read all language files from the list directory
const listDir = path.join(__dirname, 'list');
const files = fs.readdirSync(listDir).filter(file => file.endsWith('.txt'));

let allWords = [];
for (const file of files) {
    const filePath = path.join(listDir, file);
    const words = fs.readFileSync(filePath, 'utf-8').split(/\r?\n/).filter(word => word.trim() !== '');
    allWords = allWords.concat(words);
}

module.exports = allWords;
