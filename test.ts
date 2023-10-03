import { grabInputEvent } from "./src"

grabInputEvent((event) => {
    // if (event.event.type === "Wheel") {
    //     console.log(event);
    // }
    console.log(event);
    return true
})

console.log(1);
