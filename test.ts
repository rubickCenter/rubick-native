// import { onInputEvent } from "./src"


// onInputEvent((event) => {
//     console.log(event);
// })

import { shortcutWin } from "./src/sysapp/windows"

// console.log(shortcutWin());
for await (const i of shortcutWin()) {
    console.log(i);
    // sss++
}