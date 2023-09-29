import { arch, platform, homedir } from "os"
import { onClipboardChange } from "../addon"
import got from "got"
import { Extract } from "unzip-stream"
import { access, mkdir, constants } from "fs/promises"
import { join } from "path"
import { execaCommand } from "execa"

// 启动剪切板程序 linux cb; windows cb.exe; macos cb;
export default async () => {
    const repoURL = "https://ghproxy.com/https://github.com/Slackadays/Clipboard"
    let a = arch()
    let p: string = platform()
    // 确保目录存在
    const dirPath = join(homedir(), "cb")
    try {
        await access(dirPath, constants.O_DIRECTORY)
    } catch {
        await mkdir(dirPath)
    }
    // cb 路径
    const cbPath = join(dirPath, 'bin', p === "win32" ? "cb.exe" : "cb")
    try {
        await access(cbPath)
    } catch {
        switch (a) {
            case "arm64":
                break;
            case "x64":
                a = "amd64"
                break;
            default:
                throw new Error("Not Support Your Sys Arch")
        }
        switch (p) {
            case "freebsd":
            case "linux":
            case "netbsd":
            case "openbsd":
                break;
            case "win32":
                p = "windows"
                break;
            case "darwin":
                p = "macos"
                break;
            default:
                throw new Error("Not Support Your Sys Arch")
        }
        const latest = (await fetch(repoURL + "/releases/latest")).url.split("/").pop()
        const durl = repoURL + `/releases/download/${latest}/clipboard-${p}-${p === "macos" ? 'arm64-amd64' : a}.zip`
        got.stream(durl).pipe(Extract({ path: dirPath }))
    } finally {
        const execCB = async () => await execaCommand(cbPath + " info", { env: { "CLIPBOARD_SILENT": "true" } })
        // 剪切板监听
        onClipboardChange(execCB)
        // 剪切板历史文件夹
        return join((await execCB()).stdout.split(`"path": "`).at(1)?.split(`",\r\n`).at(0)!, "data")
    }
}
