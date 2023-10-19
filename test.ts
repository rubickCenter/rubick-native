import { getSystemApp } from "./src"

console.time("1")
getSystemApp((e) => {
    console.log(e);

})
console.timeEnd("1")
