import { readFileSync, writeFileSync } from "fs"

const pkg = { ...readFileSync('./package.json', 'utf8'), optionalDependencies: readFileSync('./addon/package.json', 'utf8').optionalDependencies }

writeFileSync('./package.json', pkg)