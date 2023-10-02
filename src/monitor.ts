import { onInputEvent as oie, grabInputEvent as gie } from "../addon"

export const onInputEvent = (callback: (event: object) => void) => oie((event) => callback(JSON.parse(event)))
export const grabInputEvent = (callback: (event: object) => boolean) => gie((event) => callback(JSON.parse(event)))