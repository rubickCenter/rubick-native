import { shortcutWin } from "./windows"
import { platform } from "os"
// exeLookBase64 仅限 windows 平台
const { exeLookBase64 } = await import("../../addon")

export interface Apps {
    name: string;
    description: string;
    execPath: string;
    shortCutPath: string;
    workingDir: string;
}

// todo linux/macos
export const getSystemApp = (extraPath?: string[]) => {
    switch (platform()) {
        case "win32":
            return shortcutWin(extraPath)

        // case "linux":
        //     break;

        // case "darwin":
        //     break;

        default:
            throw new Error("Your System is Not Supported");
    }
}

export const getAppIcon = (path: string): string | null => {
    switch (platform()) {
        case "win32":
            return exeLookBase64(path)

        // case "linux":
        //     break;

        // case "darwin":
        //     break;

        default:
            throw new Error("Your System is Not Supported");
    }

}