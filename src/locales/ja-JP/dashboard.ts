/**
 * @description ウィンドウ一覧/ダッシュボード - 日本語
 */
export default {
  // ページタイトル
  title: 'ウィンドウ一覧',

  // ツールバー
  toolbar: {
    createWindow: 'ウィンドウ作成',
    batchOpen: '一括開く',
    batchClose: '一括閉じる',
    batchDelete: '一括削除',
    importProfile: '設定をインポート',
    exportProfile: '設定をエクスポート',
    sortBy: '並び替え',
    filterBy: 'フィルター',
    viewMode: '表示モード',
    listView: 'リスト表示',
    gridView: 'グリッド表示'
  },

  // 並び替えオプション
  sort: {
    lastUpdated: '最近更新',
    name: '名前',
    createTime: '作成日時',
    lastOpened: '最近開いた'
  },

  // フィルターオプション
  filter: {
    all: 'すべて',
    running: '実行中',
    stopped: '停止',
    byGroup: 'グループ別',
    byTag: 'タグ別',
    byProxy: 'プロキシ別'
  },

  // ウィンドウカード
  card: {
    id: 'ID',
    group: 'グループ',
    proxy: 'プロキシ',
    tags: 'タグ',
    lastOpened: '最終起動',
    createTime: '作成日時',
    noProxy: '直接接続',
    noGroup: '未グループ',
    noTags: 'タグなし'
  },

  // ウィンドウ操作
  action: {
    open: 'ウィンドウを開く',
    close: 'ウィンドウを閉じる',
    edit: '設定を編集',
    clone: 'ウィンドウを複製',
    delete: 'ウィンドウを削除',
    moveToGroup: 'グループに移動',
    addTags: 'タグを追加',
    setProxy: 'プロキシを設定',
    viewDetail: '詳細を表示',
    copyId: 'IDをコピー'
  },

  // ウィンドウ作成ウィザード
  createWizard: {
    title: 'ウィンドウ作成',
    step1: '基本設定',
    step2: 'プロキシ設定',
    step3: 'フィンガープリント設定',
    step4: '環境設定',
    basicInfo: '基本情報',
    windowName: 'ウィンドウ名',
    windowNamePlaceholder: 'ウィンドウ名を入力',
    selectGroup: 'グループを選択',
    selectTags: 'タグを選択',
    remark: '備考',
    remarkPlaceholder: '備考を入力（任意）'
  },

  // プロキシ設定
  proxySettings: {
    title: 'プロキシ設定',
    proxyType: 'プロキシタイプ',
    directConnect: '直接接続（プロキシなし）',
    selectProxy: '既存のプロキシを選択',
    createProxy: '新規プロキシを作成',
    noProxy: 'プロキシがありません。先に作成してください'
  },

  // フィンガープリント設定
  fingerprintSettings: {
    title: 'フィンガープリント設定',
    autoGenerate: '自動生成',
    manualConfig: '手動設定',
    browserCore: 'ブラウザコア',
    userAgent: 'User Agent',
    screenResolution: '画面解像度',
    language: '言語',
    timezone: 'タイムゾーン',
    webGL: 'WebGL',
    canvas: 'Canvas',
    audio: 'オーディオフィンガープリント',
    fonts: 'フォントリスト',
    plugins: 'プラグインリスト'
  },

  // 環境設定
  preferenceSettings: {
    title: '環境設定',
    startUrl: '起動ページ',
    startUrlPlaceholder: '起動時に開くURLを入力',
    downloadPath: 'ダウンロードパス',
    cookieSync: 'Cookie 同期',
    localStorage: 'LocalStorage',
    cache: 'キャッシュポリシー'
  },

  // 空の状態
  empty: {
    title: 'ウィンドウがありません',
    description: '上のボタンをクリックして最初のブラウザウィンドウを作成',
    createButton: 'ウィンドウ作成'
  },

  // バッチ操作
  batch: {
    selected: '{count} 個のウィンドウを選択中',
    confirmDelete: '選択した {count} 個のウィンドウを削除しますか？この操作は元に戻せません！',
    confirmClose: '選択した {count} 個の実行中ウィンドウを閉じますか？',
    openSuccess: '{count} 個のウィンドウを開きました',
    closeSuccess: '{count} 個のウィンドウを閉じました',
    deleteSuccess: '{count} 個のウィンドウを削除しました'
  }
}
