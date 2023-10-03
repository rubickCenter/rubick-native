import { onInputEvent as oie, grabInputEvent as gie } from "../addon"

export type EventKeyType = "Alt" |
    "AltGr" |
    "Backspace" |
    "CapsLock" |
    "ControlLeft" |
    "ControlRight" |
    "Delete" |
    "DownArrow" |
    "End" |
    "Escape" |
    "F1" |
    "F10" |
    "F11" |
    "F12" |
    "F2" |
    "F3" |
    "F4" |
    "F5" |
    "F6" |
    "F7" |
    "F8" |
    "F9" |
    "Home" |
    "LeftArrow" |
    "MetaLeft" |
    "MetaRight" |
    "PageDown" |
    "PageUp" |
    "Return" |
    "RightArrow" |
    "ShiftLeft" |
    "ShiftRight" |
    "Space" |
    "Tab" |
    "UpArrow" |
    "PrintScreen" |
    "ScrollLock" |
    "Pause" |
    "NumLock" |
    "BackQuote" |
    "Num1" |
    "Num2" |
    "Num3" |
    "Num4" |
    "Num5" |
    "Num6" |
    "Num7" |
    "Num8" |
    "Num9" |
    "Num0" |
    "Minus" |
    "Equal" |
    "KeyQ" |
    "KeyW" |
    "KeyE" |
    "KeyR" |
    "KeyT" |
    "KeyY" |
    "KeyU" |
    "KeyI" |
    "KeyO" |
    "KeyP" |
    "LeftBracket" |
    "RightBracket" |
    "KeyA" |
    "KeyS" |
    "KeyD" |
    "KeyF" |
    "KeyG" |
    "KeyH" |
    "KeyJ" |
    "KeyK" |
    "KeyL" |
    "SemiColon" |
    "Quote" |
    "BackSlash" |
    "IntlBackslash" |
    "KeyZ" |
    "KeyX" |
    "KeyC" |
    "KeyV" |
    "KeyB" |
    "KeyN" |
    "KeyM" |
    "Comma" |
    "Dot" |
    "Slash" |
    "Insert" |
    "KpReturn" |
    "KpMinus" |
    "KpPlus" |
    "KpMultiply" |
    "KpDivide" |
    "Kp0" |
    "Kp1" |
    "Kp2" |
    "Kp3" |
    "Kp4" |
    "Kp5" |
    "Kp6" |
    "Kp7" |
    "Kp8" |
    "Kp9" |
    "KpDelete" |
    "Function" | {
        Unknown: number
    }

export type EventBtnType = "Left" |
    "Right" |
    "Middle" | {
        Unknown: number
    }

export interface MouseKeyBoardEventOther {
    time: {
        secs_since_epoch: number,
        nanos_since_epoch: number
    },
    name: null,
    event: {
        type: "KeyRelease" | "ButtonPress" | "ButtonRelease"
        value: EventKeyType
    } | {
        type: "MouseMove"
        value: { x: number, y: number }
    } | {
        type: "Wheel",
        value: { delta_x: number, delta_y: number }
    }
}

export interface MouseKeyBoardEventKeyPress {
    time: {
        secs_since_epoch: number,
        nanos_since_epoch: number
    },
    name: string,
    event: {
        type: "KeyPress"
        value: EventKeyType
    }
}

export type MouseKeyBoardEvent = MouseKeyBoardEventOther | MouseKeyBoardEventKeyPress
const parse = (e: string) => {
    const event = JSON.parse(e)
    const [type, value] = Object.entries(event.event_type).pop()!
    return {
        time: event.time,
        name: event.name,
        event: { type, value }
    } as MouseKeyBoardEvent
}
export const onInputEvent = (callback: (event: MouseKeyBoardEvent) => void) => oie((event) => callback(parse(event)))
export const grabInputEvent = (callback: (event: MouseKeyBoardEvent) => boolean) => gie((event) => callback(parse(event)))