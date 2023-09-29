import {
    Position,
    sendKeyboardSimulation as ks,
    sendMouseSimulation as ms,
} from "../addon";

export type MouseBtn =
    | "Left"
    | "Middle"
    | "Right"
    | "Back"
    | "Forward"

export interface MoveMoveInput {
    type: "relative" | "absolute";
    data: Position;
}

// example: {+CTRL}a{-CTRL}{+SHIFT}Hello World{-SHIFT}
// 所有可用键 https://github.com/enigo-rs/enigo/blob/master/src/keycodes.rs
export const sendKeyboardSimulation = (cmd: string) => ks(cmd);

export const mouseScrollX = (len: number) => {
    ms({ action: 3, data: { x: len, y: 0 } });
}

export const mouseScrollY = (len: number) => {
    ms({ action: 4, data: { x: 0, y: len } });
};

export const mouseMove = (input: MoveMoveInput) => {
    ms({ action: input.type === "absolute" ? 2 : 1, data: input.data });
};

export const mouseLocaion = () => ms({ action: 0 })

const mouseDUC = (btn: MouseBtn, action: 5 | 6 | 7) => {
    let button = 0;
    switch (btn) {
        case "Left":
            break;
        case "Middle":
            button = 1;
            break;
        case "Right":
            button = 2;
            break;
        case "Back":
            button = 3;
            break;
        case "Forward":
            button = 4;
            break;
        default:
            break;
    }
    ms({ action, button });
};

export const mouseDown = (btn: MouseBtn) => mouseDUC(btn, 6)
export const mouseUp = (btn: MouseBtn) => mouseDUC(btn, 5)
export const mouseClick = (btn: MouseBtn) => mouseDUC(btn, 7)
