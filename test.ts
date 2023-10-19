import { existsSync } from "fs";
import { getSystemApp } from "./src"

console.time("1")
await getSystemApp((e) => {
    if (!existsSync(e.execPath)) {
        console.log(e);
    }
})
console.timeEnd("1")
