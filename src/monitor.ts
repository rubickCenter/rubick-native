import { onInputEvent as oie } from "../addon"

export const onInputEvent = (callback: (event: string) => void) => oie((event) => callback(JSON.parse(event)))