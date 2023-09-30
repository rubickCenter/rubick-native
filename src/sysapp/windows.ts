import { join, parse } from "path";
import { homedir } from "os"
import { fdir } from "fdir";
import { parseLnkFallback, parseLnk } from "../../addon"
import { Apps } from ".";

export const shortcutWin = (extraPath: string[] = []) => ({
    [Symbol.asyncIterator]: async function* () {
        const hdir = homedir()
        const f = new fdir().glob("./**/*.lnk").withFullPaths()
        const defaultPaths = [
            join(process.env.ProgramData, "/Microsoft/Windows/Start Menu/Programs"),
            join(process.env.AppData, "/Microsoft/Windows/Start Menu/Programs"),
            join(process.env.PUBLIC, 'Desktop'),
            join(hdir, 'Desktop'),
            ...extraPath
        ]
        for (const p of defaultPaths) {
            const o = await f.crawl(p).withPromise()
            for (const t of o) {
                const data = parseLnk(t)
                const { name, dir } = parse(t)

                if (data) {
                    const d = JSON.parse(data)
                    yield ({
                        name,
                        description: d.name_string ?? null,
                        execPath: d.target_full_path,
                        shortCutPath: t,
                        workingDir: d.working_dir ?? null,
                    }) as Apps
                } else {
                    const d = parseLnkFallback(t)
                    const execPath = join(dir, d.relativePath ?? '')

                    yield ({
                        name,
                        description: d.nameString ?? null,
                        execPath,
                        shortCutPath: t,
                        workingDir: d.workingDir ?? null,
                    }) as Apps
                }
            }
        }
    }
})
