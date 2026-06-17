const { TextCensor, allWords } = require('./lib.js');

const censor = new TextCensor(allWords);

const text = "This is a stupid test with some badwords and ugly things. Dasar anjing kau! Puta madre!";
console.log("Original text:", text);
const cleaned = censor.clean(text);
console.log("Cleaned text:", cleaned);

censor.addWords(['heck']);
const moreCleaned = censor.clean("What the heck is this badword?");
console.log("More cleaned text:", moreCleaned);
