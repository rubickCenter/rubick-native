const { writeFileSync } = require("fs")
const { version } = require("../package.json")
const pkg = require("./package.json")
pkg.version = version
writeFileSync("./package.json", JSON.stringify(pkg))
