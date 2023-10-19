import { activeWindow } from "@miniben90/x-win"
import { lstat } from "fs/promises"
import { homedir } from "os"
import { join } from "path"

// 获取活动的文件夹路径
export const getFolderOpenPath = async () => {
    if (process.platform === 'darwin') {
        const { execa } = await import("execa")
        const res = await execa('osascript', ['-e', `
			tell app "Finder"
				try
					POSIX path of (insertion location as alias)
				on error
					POSIX path of (path to desktop folder as alias)
				end try
			end tell
		`])
        return res.stdout;
    }

    if (process.platform === 'win32') {
        const win = activeWindow()
        if (win.info.execName === 'explorer') {
            const base = homedir()
            let path: string
            switch (win.title) {
                case 'Home':
                case '主文件夹':
                    path = base
                    break;

                case 'Downloads':
                case '下载':
                    path = join(base, 'Downloads')
                    break;

                case 'Documents':
                case '文档':
                    path = join(base, 'Documents')
                    break;

                case 'Desktop':
                case '桌面':
                    path = join(base, 'Desktop')
                    break;

                case 'Videos':
                case '视频':
                    path = join(base, 'Videos')
                    break;

                case 'Pictures':
                case '图片':
                    path = join(base, 'Pictures')
                    break;

                case 'Music':
                case '音乐':
                    path = join(base, 'Music')
                    break;

                case 'Links':
                case '链接':
                    path = join(base, 'Music')
                    break;

                default:
                    path = win.title
                    break;
            }
            try {
                const s = await lstat(path)
                if (s.isDirectory()) {
                    return path
                }
            } catch {
                return null
            }
        }
    }

    // todo linux
    return null
}