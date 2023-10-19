import { shortcutWin } from "./windows"
import { platform } from "os"
import { exeLookBase64 } from "../../addon"

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

export const getAppIcon = (path: string): string => {
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