import { getClipboardContent as gcbc } from "addon"

interface ClipboardContentText {
    type: "text",
    content: string
}

interface ClipboardContentFile {
    type: "file",
    content: string[]
}

export type ClipboardContent = ClipboardContentText | ClipboardContentFile | null

export const getClipboardContent = (): ClipboardContent => {
    const c = gcbc()
    if (c?.type === 'text') {
        return {
            type: "text",
            content: c.content.at(0)
        }
    } else {
        return c as ClipboardContent
    }
}