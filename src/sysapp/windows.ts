import { join, parse } from "path";
import { homedir } from "os"
import { fdir } from "fdir";
import { parseLnkFallback, parseLnk } from "../../addon"
import { CallBack } from ".";

export const shortcutWin = async (callback: CallBack, extraPath: string[] = []) => {
    const hdir = homedir()
    const f = new fdir().glob("./**/*.lnk").withFullPaths()
        .filter((t) => {
            const { name, dir } = parse(t)
            try {
                const data = parseLnk(t)
                const d = JSON.parse(data)
                callback({
                    name,
                    description: d.name_string ?? null,
                    execPath: d.target_full_path,
                    shortCutPath: t,
                    workingDir: d.working_dir ?? null,
                })
            } catch {
                const d = parseLnkFallback(t)
                const execPath = join(dir, d.relativePath ?? '')
                callback({
                    name,
                    description: d.nameString ?? null,
                    execPath,
                    shortCutPath: t,
                    workingDir: d.workingDir ?? null,
                })
            }
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
