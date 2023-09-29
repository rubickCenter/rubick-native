import { arch, platform, homedir } from "os"
import { onClipboardChange } from "../../addon"
import got from "got"
import { Extract } from "unzip-stream"
import { access, mkdir, constants } from "fs/promises"
import { join } from "path"
import { execaCommand } from "execa"
const getKey = (stdout: string, key: string) => stdout.split(`"${key}": `).at(1)?.split(`,\r\n`).at(0)!
import { asyncFolderWalker } from "async-folder-walker"

// 启动剪切板程序
export default async () => {
    let latestNum = 0
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
    // 同步剪切板内容
    const execCB = async () => {
        const stdout = (await execaCommand(cbPath + " info", { env: { "CLIPBOARD_SILENT": "true" } })).stdout
        // 最新缓存
        latestNum = Number(getKey(stdout, "totalEntries")) - 1
        return stdout
    }
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
        // 剪切板历史路径
        const basePath = join(getKey(await execCB(), "path").replaceAll('"', ''), "data")
        // 剪切板监听
        onClipboardChange(execCB)
        return {
            latest: () => {
                const latestPath = join(basePath, latestNum.toString())
                const walker = asyncFolderWalker(latestPath, { maxDepth: 0 })
                return walker
            }
        }
    }
}
