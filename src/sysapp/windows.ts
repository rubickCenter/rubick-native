import { join, parse } from "path";
import { homedir } from "os"
import { fdir } from "fdir";
import { parseLnk3, parseLnk } from "../../addon"

// todo lnk icon
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
                const { name } = parse(t)

                if (data) {
                    const d = JSON.parse(data)
                    const path = join(t, d.relativePath ?? '')
                    yield ({
                        name,
                        path,
                        description: d.name_string ?? null,
                        working_dir: d.working_dir ?? null,
                        // icon_location: d.icon_location ?? null
                    })
                } else {
                    const d = parseLnk3(t)
                    const path = join(t, d.relativePath ?? '')
                    yield ({
                        name,
                        path,
                        description: d.nameString ?? null,
                        working_dir: d.workingDir ?? null,
                        // icon_location: d.iconLocation ?? null
                    })
                }
            }
        }
    }
})

// let sss = 0
// for await (const i of shortcutWin()) {
//     console.log(i);
//     sss++
// }
// console.log(sss);