# 浏览器内核资源目录

> 编译后的自研魔改 Chromium 146 内核存放于此

## 目录结构

```
resources/
└── kernel/
    ├── win32/              # Windows x64 内核
    │   ├── chrome.exe      # 主程序入口
    │   ├── chrome.dll      # 核心库
    │   ├── *.dll           # 其他依赖
    │   ├── locales/        # 语言包
    │   ├── resources.pak
    │   └── ...
    ├── darwin/             # macOS 内核 (未来)
    └── linux/              # Linux 内核 (未来)
```

## 使用说明

1. 将编译好的 Chromium 整个目录复制到对应平台文件夹
2. 确保 `chrome.exe` (Windows) 位于 `win32/` 根目录
3. 不要修改文件结构，启动器会自动定位

## 注意事项

- 此目录会被 Tauri 打包进最终应用
- 内核文件较大 (~150MB)，请勿提交到 Git
- 添加到 `.gitignore` 中排除

## 版本管理

| 平台 | 内核版本 | 更新日期 |
|------|----------|----------|
| Windows | - | 待放置 |
| macOS | - | 未开发 |
| Linux | - | 未开发 |
