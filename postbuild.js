const { readFileSync, writeFileSync } = require("fs")
const path = require("path");

const ijs_path = path.join(__dirname, "index.js")
const ijs = readFileSync(ijs_path, "utf-8")
const twoijs = ijs.split('binding\`)\n}\n')

const newijs = 'const { join } = require("path")\nconst nativeBinding = (new Function(`require`,`__dirname`,`' + twoijs[0].replaceAll('`', '\\`').replaceAll('$', '\\$') + 'binding\\`)\n}\nreturn nativeBinding`))((path)=>require(path.replace("./","../")),join(__dirname,".."))\n' + twoijs[1]

writeFileSync(ijs_path, newijs);

(async () => await require("tsup").build({
    "entry": [
        "lib/index.ts"
    ],
    "cjsInterop": true,
    "format": [
        "cjs",
        "esm"
    ],
    "clean": true,
    "dts": true,
    "treeshake": true,
    "shims": true
}))()