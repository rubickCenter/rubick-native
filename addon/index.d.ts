/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface ClipBoardContentJson {
  type: 'file' | 'text'
  content: Array<string>
}
export function getClipboardContent(): ClipBoardContentJson | null
export function onInputEvent(callback: (event: string) => void): void
export function grabInputEvent(callback: (event: string) => boolean): void
export function exeLookBase64(fileName: string): string | null
export function parseLnk(path: string): string | null
export function parseLnkFallback(path: string): LnkData
export function sendKeyboardSimulation(cmd: string): void
export const enum MouseBtn {
  Left = 0,
  Middle = 1,
  Right = 2,
  Back = 3,
  Forward = 4
}
export const enum MouseAction {
  Locaion = 0,
  MoveRelative = 1,
  MoveTo = 2,
  ScrollX = 3,
  ScrollY = 4,
  Up = 5,
  Down = 6,
  Click = 7
}
export interface MouseActionInput {
  action: MouseAction
  data?: Position
  button?: MouseBtn
}
export interface Position {
  x: number
  y: number
}
export function sendMouseSimulation(input: MouseActionInput): Position | null
export class ShorCutImg {
  data: Array<number>
  width: number
  height: number
}
export class LnkData {
  nameString?: string
  relativePath?: string
  workingDir?: string
  iconLocation?: string
}