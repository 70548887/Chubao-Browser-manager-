/**
 * @description 設定 - 日本語
 */
export default {
  // ページタイトル
  title: '設定',

  // 設定カテゴリ
  category: {
    general: '一般設定',
    appearance: '外観',
    language: '言語',
    browser: 'ブラウザ',
    proxy: 'プロキシ',
    storage: 'ストレージ',
    notification: '通知',
    security: 'セキュリティ',
    about: 'このアプリについて'
  },

  // 一般設定
  general: {
    title: '一般設定',
    startOnBoot: 'システム起動時に自動起動',
    startOnBootDesc: 'システム起動時にアプリを自動的に起動',
    minimizeToTray: 'トレイに最小化',
    minimizeToTrayDesc: 'ウィンドウを閉じるとシステムトレイに最小化',
    checkUpdate: '自動更新チェック',
    checkUpdateDesc: '起動時に新しいバージョンを確認',
    analytics: '使用統計',
    analyticsDesc: '製品改善のために匿名の使用データを送信'
  },

  // 外観設定
  appearance: {
    title: '外観設定',
    theme: 'テーマ',
    themeLight: 'ライト',
    themeDark: 'ダーク',
    themeAuto: 'システムに従う',
    accentColor: 'アクセントカラー',
    fontSize: 'フォントサイズ',
    fontSizeSmall: '小',
    fontSizeNormal: '標準',
    fontSizeLarge: '大',
    sidebarWidth: 'サイドバー幅',
    compactMode: 'コンパクトモード',
    compactModeDesc: 'UI要素の間隔を狭める'
  },

  // 言語設定
  language: {
    title: '言語設定',
    displayLanguage: '表示言語',
    selectLanguage: '言語を選択',
    languageZhCN: '简体中文',
    languageEnUS: 'English',
    languageJaJP: '日本語',
    languageKoKR: '한국어',
    restartRequired: '言語変更後、アプリの再起動が必要です'
  },

  // ブラウザ設定
  browser: {
    title: 'ブラウザ設定',
    kernelPath: 'カーネルパス',
    kernelPathDesc: 'ブラウザカーネルのディレクトリ',
    selectPath: 'パスを選択',
    defaultProfile: 'デフォルト設定',
    cacheSize: 'キャッシュサイズ制限',
    clearCache: 'キャッシュをクリア',
    clearCacheDesc: 'すべてのブラウザキャッシュデータを削除'
  },

  // プロキシ設定
  proxy: {
    title: 'プロキシ設定',
    globalProxy: 'グローバルプロキシ',
    globalProxyDesc: 'すべてのウィンドウにデフォルトプロキシを設定',
    noProxy: 'プロキシなし',
    selectProxy: 'プロキシを選択',
    timeout: '接続タイムアウト',
    timeoutDesc: 'プロキシ接続のタイムアウト時間（秒）',
    retryCount: 'リトライ回数',
    retryCountDesc: '接続失敗時のリトライ回数'
  },

  // ストレージ設定
  storage: {
    title: 'ストレージ設定',
    dataPath: 'データ保存パス',
    dataPathDesc: 'ウィンドウ設定とデータの保存場所',
    usedSpace: '使用容量',
    totalSpace: '合計容量',
    cleanUp: 'クリーンアップ',
    cleanUpDesc: '一時ファイルと不要なデータを削除',
    export: 'データをエクスポート',
    exportDesc: 'すべてのウィンドウ設定と設定をエクスポート',
    import: 'データをインポート',
    importDesc: 'バックアップファイルからデータを復元'
  },

  // 通知設定
  notification: {
    title: '通知設定',
    enableNotification: '通知を有効',
    enableNotificationDesc: 'システム通知を表示',
    sound: '通知音',
    soundDesc: '通知音を再生',
    taskComplete: 'タスク完了通知',
    taskCompleteDesc: 'RPAタスク完了時に通知',
    updateAvailable: '更新通知',
    updateAvailableDesc: '新バージョンがある場合に通知'
  },

  // セキュリティ設定
  security: {
    title: 'セキュリティ設定',
    password: 'アクセスパスワード',
    passwordDesc: 'アプリ起動時のパスワードを設定',
    setPassword: 'パスワードを設定',
    changePassword: 'パスワードを変更',
    removePassword: 'パスワードを削除',
    autoLock: '自動ロック',
    autoLockDesc: 'アイドル状態で一定時間後に自動ロック',
    lockTime: 'ロック時間',
    encryption: 'データ暗号化',
    encryptionDesc: '機密データを暗号化して保存'
  },

  // このアプリについて
  about: {
    title: 'このアプリについて',
    appName: 'ブラウザマルチオープナー',
    version: 'バージョン',
    buildTime: 'ビルド日時',
    copyright: '著作権情報',
    website: '公式サイト',
    documentation: 'ドキュメント',
    feedback: 'フィードバック',
    checkUpdate: '更新を確認',
    currentVersion: '現在のバージョンは最新です',
    newVersion: '新バージョン {version} が利用可能です',
    changelog: '更新履歴'
  },

  // メッセージ
  message: {
    saveSuccess: '設定を保存しました',
    restartRequired: '一部の設定はアプリ再起動後に有効になります',
    exportSuccess: 'データをエクスポートしました',
    importSuccess: 'データをインポートしました',
    cleanUpSuccess: 'クリーンアップ完了、{size}を解放しました',
    passwordSet: 'パスワードを設定しました',
    passwordChanged: 'パスワードを変更しました',
    passwordRemoved: 'パスワードを削除しました'
  }
}
