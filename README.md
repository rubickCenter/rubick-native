# Rubick Addon

封装常用跨平台能力

## 贡献

1. Fork & Clone 本项目
2. 贡献 Rust 部分: 进入 addon 文件下, 基于 [napi](https://napi.rs/) 教程进行开发; 使用 `pnpm build` 命令打包生成 npm 包
3. 贡献 JavaScript 部分: 在根目录下进行开发, 通过 `import { func } from "../addon"` 调用 Rust 函数
4. 提起 PR

## API

### 1. Keyborad & Mouse ✅

1. mouseClick: 鼠标点击
2. mouseDown: 鼠标按下
3. mouseUp: 鼠标抬起
4. mouseLocaion: 获取鼠标位置

##### 出参 eg

```js
{ x: 485, y: 449 }
```

5. mouseMove: 鼠标移动
6. mouseScrollX: 鼠标滚轮左右滑动(正值向右,负值向左)
7. mouseScrollY: 鼠标滚轮上下滑动(正值向下,负值向上)
8. sendKeyboardSimulation: 模拟键盘输入

##### 入参 eg

```js
// - 为按下, - 为抬起；所有可用键 https://github.com/enigo-rs/enigo/blob/master/src/keycodes.rs
 {+CTRL}a{-CTRL}{+SHIFT}Hello World{-SHIFT}
```

9. 监听输入事件

##### 入参 eg

```js
(event: object) => console.log(event);
```

- callback: function 监听输入事件的函数

##### event object eg

```json
{
  "time": { "secs_since_epoch": 1695999163, "nanos_since_epoch": 631148700 }, // 触发时间
  "name": "a", // 输入内容
  "event_type": { "KeyPress": "KeyA" } // 事件类型
}
```

### 2. Clipboard ✅

可以获取剪贴板中复制的内容：

#### getClipboardContent()

##### 出参 eg

```js
// type: file
{
  type: 'file',
  content: [
    'C:/Download/test.txt',
    'C:/Download/mywork',
  ]
}
// type: text
{
  type: 'text',
  content: 'hello world'
}
```

- type: 'file' | 'text'
- content: Array<string> | string

### 3. 获取系统已安装的 app ✅(Mac/Linux 暂未支持)

原理：解析系统中所有存放快捷方式的文件夹，解析快捷方式生成应用列表

#### getSystemApp()

获取系统安装的 APP 列表

##### 入参 eg

- extraPath: 额外需要检索应用的文件夹

##### 出参 eg

异步迭代器

```js
let appNum = 0;
for await (const app of getSystemApp()) {
  console.log(app);
  appNum++;
}
console.log("Total", appNum, "App installed.");
```

app object:

```json
{
  "name": "TIM",
  "description": null,
  "execPath": "C:\\Program Files (x86)\\Tencent\\TIM\\Bin\\QQScLauncher.exe",
  "shortCutPath": "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\腾讯软件\\TIM\\TIM.lnk",
  "workingDir": "C:\\Program Files (x86)\\Tencent\\TIM\\Bin"
}
```

- name: 应用的名称
- description: 应用详情
- execPath: 应用的路径
- shortCutPath: 应用快捷方式地址
- workingDir: 应用工作目录

#### getAppIcon()

获取 APP 的图标

##### 入参 eg

- path: string windows 平台下填写 exe 文件地址;

##### 出参 eg

- img: string 图标的 base64 编码

### 4. 获取当前活跃窗口 ✅

1. getActiveWin: 获取当前活跃窗口
2. getOpenWin: 获取所有打开的窗口信息

##### 出参 eg

```json
{
  "id": 13200, // 进程 pid
  "os": "win32", // 运行平台
  "title": "test.ts - rubick-native - Visual Studio Code", // 窗口标题
  "position": { "x": -7, "y": -7, "width": 1550, "height": 830 }, // 窗口位置大小
  "info": {
    "processId": 13200, // 进程 pid
    "path": "C:\\Program Files\\Microsoft VS Code\\Code.exe", // 进程地址
    "name": "Visual Studio Code", // 进程名称
    "execName": "Code" // 进程名称
  },
  "usage": { "memory": 111030272 }, // 进程内存占用
  "url": "" // 进程关联 URL
}
```

### 5. 获取 `finder`(macos) 或者 `explorer`(win) 文件夹当前打开的路径 ✅(Linux 暂未支持)

1. getFolderOpenPath

##### 出参 eg

```js
"C://Download/mywork";
```

- path: string 返回打开的文件夹路径
