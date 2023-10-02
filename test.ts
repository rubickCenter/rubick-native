import { grabInputEvent } from "./addon"

grabInputEvent((event) => {
    const e = JSON.parse(event)?.event_type
    if (e?.ButtonRelease === "Right") {
        return false
    }
    console.log(e);
    return true
})

console.log(1);
