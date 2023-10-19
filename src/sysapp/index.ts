import { shortcutWin } from "./windows"
import { platform } from "os"
import { exeLookBase64 } from "../../addon"
import { ParsedPath } from "path";

export type CallBack = (app: App) => void | Promise<void>

export interface App extends ParsedPath {
    name: string;
    description: string;
    execPath: string;
    shortCutPath: string;
    workingDir: string;
}

// todo linux/macos
export const getSystemApp = async (callback: CallBack, extraPath?: string[]) => {
    switch (platform()) {
        case "win32":
            return await shortcutWin(callback, extraPath)

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