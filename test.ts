import { grabInputEvent } from "./src"

grabInputEvent((event) => {
    console.log(event);
    return true
})

console.log(1);
