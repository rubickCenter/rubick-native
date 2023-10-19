import { join, parse } from "path";
import { homedir } from "os"
import { fdir } from "fdir";
import { parseLnk } from "../../addon"
import { CallBack } from ".";

export const shortcutWin = async (callback: CallBack, extraPath: string[] = []) => {
    const hdir = homedir()
    const f = new fdir().glob("./**/*.lnk").withFullPaths()
        .filter((t) => {
            const d = parseLnk(t)
            callback({
                ...parse(t),
                description: d.nameString,
                execPath: d.fullPath,
                shortCutPath: t,
                workingDir: d.workingDir
            })
            return true
        })
    const defaultPaths = [
        join(process.env.ProgramData, "/Microsoft/Windows/Start Menu/Programs"),
        join(process.env.AppData, "/Microsoft/Windows/Start Menu/Programs"),
        join(process.env.PUBLIC, 'Desktop'),
        join(hdir, 'Desktop'),
        ...extraPath
    ]
    await Promise.allSettled(defaultPaths.map(path => f.crawl(path).withPromise()))
}
