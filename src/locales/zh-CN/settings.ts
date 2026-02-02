/**
 * @description 设置 - 中文
 */
export default {
  // 页面标题
  title: '设置',

  // 设置分类
  category: {
    general: '通用设置',
    appearance: '外观',
    language: '语言',
    browser: '浏览器',
    proxy: '代理',
    storage: '存储',
    notification: '通知',
    security: '安全',
    about: '关于'
  },

  // 通用设置
  general: {
    title: '通用设置',
    startOnBoot: '开机自启动',
    startOnBootDesc: '系统启动时自动运行应用',
    minimizeToTray: '最小化到托盘',
    minimizeToTrayDesc: '关闭窗口时最小化到系统托盘',
    checkUpdate: '自动检查更新',
    checkUpdateDesc: '启动时检查是否有新版本',
    analytics: '使用统计',
    analyticsDesc: '发送匿名使用数据帮助改进产品'
  },

  // 外观设置
  appearance: {
    title: '外观设置',
    theme: '主题',
    themeLight: '浅色',
    themeDark: '深色',
    themeAuto: '跟随系统',
    accentColor: '强调色',
    fontSize: '字体大小',
    fontSizeSmall: '小',
    fontSizeNormal: '标准',
    fontSizeLarge: '大',
    sidebarWidth: '侧边栏宽度',
    compactMode: '紧凑模式',
    compactModeDesc: '减少界面元素间距'
  },

  // 语言设置
  language: {
    title: '语言设置',
    displayLanguage: '显示语言',
    selectLanguage: '选择语言',
    languageZhCN: '简体中文',
    languageEnUS: 'English',
    languageJaJP: '日本語',
    languageKoKR: '한국어',
    restartRequired: '更改语言后需要重启应用'
  },

  // 浏览器设置
  browser: {
    title: '浏览器设置',
    kernelPath: '内核路径',
    kernelPathDesc: '浏览器内核所在目录',
    selectPath: '选择路径',
    defaultProfile: '默认配置',
    cacheSize: '缓存大小限制',
    clearCache: '清除缓存',
    clearCacheDesc: '清除所有浏览器缓存数据'
  },

  // 代理设置
  proxy: {
    title: '代理设置',
    globalProxy: '全局代理',
    globalProxyDesc: '为所有窗口设置默认代理',
    noProxy: '无代理',
    selectProxy: '选择代理',
    timeout: '连接超时',
    timeoutDesc: '代理连接超时时间（秒）',
    retryCount: '重试次数',
    retryCountDesc: '连接失败后的重试次数'
  },

  // 存储设置
  storage: {
    title: '存储设置',
    dataPath: '数据存储路径',
    dataPathDesc: '窗口配置和数据存储位置',
    usedSpace: '已用空间',
    totalSpace: '总空间',
    cleanUp: '清理',
    cleanUpDesc: '清理临时文件和无用数据',
    export: '导出数据',
    exportDesc: '导出所有窗口配置和设置',
    import: '导入数据',
    importDesc: '从备份文件恢复数据'
  },

  // 通知设置
  notification: {
    title: '通知设置',
    enableNotification: '启用通知',
    enableNotificationDesc: '显示系统通知',
    sound: '通知声音',
    soundDesc: '播放通知提示音',
    taskComplete: '任务完成通知',
    taskCompleteDesc: 'RPA任务完成时通知',
    updateAvailable: '更新通知',
    updateAvailableDesc: '有新版本时通知'
  },

  // 安全设置
  security: {
    title: '安全设置',
    password: '访问密码',
    passwordDesc: '设置应用启动密码',
    setPassword: '设置密码',
    changePassword: '修改密码',
    removePassword: '移除密码',
    autoLock: '自动锁定',
    autoLockDesc: '空闲一段时间后自动锁定',
    lockTime: '锁定时间',
    encryption: '数据加密',
    encryptionDesc: '加密存储敏感数据'
  },

  // 关于
  about: {
    title: '关于',
    appName: '浏览器多开器',
    version: '版本',
    buildTime: '构建时间',
    copyright: '版权信息',
    website: '官方网站',
    documentation: '文档',
    feedback: '反馈问题',
    checkUpdate: '检查更新',
    currentVersion: '当前已是最新版本',
    newVersion: '发现新版本 {version}',
    changelog: '更新日志'
  },

  // 消息提示
  message: {
    saveSuccess: '设置保存成功',
    restartRequired: '部分设置需要重启应用后生效',
    exportSuccess: '数据导出成功',
    importSuccess: '数据导入成功',
    cleanUpSuccess: '清理完成，释放了 {size} 空间',
    passwordSet: '密码设置成功',
    passwordChanged: '密码修改成功',
    passwordRemoved: '密码已移除'
  }
}
