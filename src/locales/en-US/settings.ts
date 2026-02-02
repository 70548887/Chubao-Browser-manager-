/**
 * @description Settings - English
 */
export default {
  title: 'Settings',

  category: {
    general: 'General',
    appearance: 'Appearance',
    language: 'Language',
    browser: 'Browser',
    proxy: 'Proxy',
    storage: 'Storage',
    notification: 'Notifications',
    security: 'Security',
    about: 'About'
  },

  general: {
    title: 'General',
    startOnBoot: 'Start on Boot',
    startOnBootDesc: 'Launch app when system starts',
    minimizeToTray: 'Minimize to Tray',
    minimizeToTrayDesc: 'Minimize to system tray when closing',
    checkUpdate: 'Auto Check Updates',
    checkUpdateDesc: 'Check for updates on startup',
    analytics: 'Usage Analytics',
    analyticsDesc: 'Send anonymous usage data'
  },

  appearance: {
    title: 'Appearance',
    theme: 'Theme',
    themeLight: 'Light',
    themeDark: 'Dark',
    themeAuto: 'System',
    accentColor: 'Accent Color',
    fontSize: 'Font Size',
    fontSizeSmall: 'Small',
    fontSizeNormal: 'Normal',
    fontSizeLarge: 'Large',
    sidebarWidth: 'Sidebar Width',
    compactMode: 'Compact Mode',
    compactModeDesc: 'Reduce spacing between elements'
  },

  language: {
    title: 'Language',
    displayLanguage: 'Display Language',
    selectLanguage: 'Select Language',
    languageZhCN: '简体中文',
    languageEnUS: 'English',
    languageJaJP: '日本語',
    languageKoKR: '한국어',
    restartRequired: 'Restart required after changing language'
  },

  browser: {
    title: 'Browser',
    kernelPath: 'Kernel Path',
    kernelPathDesc: 'Browser kernel directory',
    selectPath: 'Select Path',
    defaultProfile: 'Default Profile',
    cacheSize: 'Cache Size Limit',
    clearCache: 'Clear Cache',
    clearCacheDesc: 'Clear all browser cache'
  },

  proxy: {
    title: 'Proxy',
    globalProxy: 'Global Proxy',
    globalProxyDesc: 'Default proxy for all profiles',
    noProxy: 'No Proxy',
    selectProxy: 'Select Proxy',
    timeout: 'Connection Timeout',
    timeoutDesc: 'Proxy connection timeout (seconds)',
    retryCount: 'Retry Count',
    retryCountDesc: 'Retries after connection failure'
  },

  storage: {
    title: 'Storage',
    dataPath: 'Data Path',
    dataPathDesc: 'Profile and data storage location',
    usedSpace: 'Used Space',
    totalSpace: 'Total Space',
    cleanUp: 'Clean Up',
    cleanUpDesc: 'Clean temporary files',
    export: 'Export Data',
    exportDesc: 'Export all profiles and settings',
    import: 'Import Data',
    importDesc: 'Restore from backup'
  },

  notification: {
    title: 'Notifications',
    enableNotification: 'Enable Notifications',
    enableNotificationDesc: 'Show system notifications',
    sound: 'Sound',
    soundDesc: 'Play notification sound',
    taskComplete: 'Task Complete',
    taskCompleteDesc: 'Notify when RPA task completes',
    updateAvailable: 'Update Available',
    updateAvailableDesc: 'Notify when new version is available'
  },

  security: {
    title: 'Security',
    password: 'Password',
    passwordDesc: 'Set app launch password',
    setPassword: 'Set Password',
    changePassword: 'Change Password',
    removePassword: 'Remove Password',
    autoLock: 'Auto Lock',
    autoLockDesc: 'Lock after idle time',
    lockTime: 'Lock Time',
    encryption: 'Data Encryption',
    encryptionDesc: 'Encrypt sensitive data'
  },

  about: {
    title: 'About',
    appName: 'Browser Manager',
    version: 'Version',
    buildTime: 'Build Time',
    copyright: 'Copyright',
    website: 'Website',
    documentation: 'Documentation',
    feedback: 'Feedback',
    checkUpdate: 'Check Update',
    currentVersion: 'You are up to date',
    newVersion: 'New version {version} available',
    changelog: 'Changelog'
  },

  message: {
    saveSuccess: 'Settings saved',
    restartRequired: 'Some changes require restart',
    exportSuccess: 'Data exported',
    importSuccess: 'Data imported',
    cleanUpSuccess: 'Cleaned up {size}',
    passwordSet: 'Password set',
    passwordChanged: 'Password changed',
    passwordRemoved: 'Password removed'
  }
}
