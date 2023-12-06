const { readFileSync, writeFileSync } = require("fs")
const path = require("path");

const ijs_path = path.join(__dirname, "index.js")
const ijs = readFileSync(ijs_path, "utf-8")
const twoijs = ijs.split('\n\nmodule.exports')
const newijs = 'const nativeBinding = (new Function(`require`,`__dirname`,`' + twoijs[0].replaceAll('`', '\\`').replaceAll('$', '\\$') + '\nreturn nativeBinding`))(require,__dirname)\nmodule.exports' + twoijs[1]

writeFileSync(ijs_path, newijs)