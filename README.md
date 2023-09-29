## rubick rust addon

## API

### 1. Keyborad

可以直接参考 robotjs 实现：http://robotjs.io/docs/syntax#keyboard

### 2. Clipboard

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

### 3. 获取系统已安装的 app

#### getSystemApp()

##### 出参 eg

```js
[
  {
    desc: "C://Download/wechat.exe",
    type: "app",
    icon: "base64",
    name: "wechat",
  },
];
```

- desc: 应用的路径
- type: 应用类型
- icon: 64 \* 64 的应用 icon
- name: 应用的名称

### 4. 获取当前活跃窗口 ✅

#### getActiveWin()

参考实现：https://github.com/sindresorhus/active-win

### 5. 获取 `finder`(macos) 或者 `explorer`(win) 文件夹当前打开的路径 ✅

#### getFolderOpenPath()

##### 出参 eg

```js
"C://Download/mywork";
```

- path: string 返回打开的文件夹路径
