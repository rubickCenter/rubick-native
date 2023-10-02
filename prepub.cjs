const { writeFileSync } = require("fs")
const { optionalDependencies } = require("./addon/package.json")
const pkg = require("./package.json")
if (optionalDependencies) {
    pkg.optionalDependencies = optionalDependencies
    writeFileSync("./package.json", JSON.stringify(pkg))
}
