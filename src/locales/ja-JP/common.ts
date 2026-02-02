/**
 * @description 共通用語 - 日本語
 */
export default {
  // アクションボタン
  action: {
    add: '新規作成',
    create: '作成',
    edit: '編集',
    delete: '削除',
    remove: '削除',
    save: '保存',
    cancel: 'キャンセル',
    confirm: '確認',
    close: '閉じる',
    search: '検索',
    filter: 'フィルター',
    sort: '並び替え',
    refresh: '更新',
    reset: 'リセット',
    export: 'エクスポート',
    import: 'インポート',
    copy: 'コピー',
    paste: '貼り付け',
    selectAll: 'すべて選択',
    batchDelete: '一括削除',
    batchOpen: '一括開く',
    batchClose: '一括閉じる',
    start: '開始',
    stop: '停止',
    open: '開く',
    view: '表示',
    detail: '詳細',
    back: '戻る',
    next: '次へ',
    prev: '前へ',
    submit: '送信',
    apply: '適用',
    test: 'テスト',
    check: '検出',
    retry: '再試行',
    more: 'もっと見る',
    expand: '展開',
    collapse: '折りたたむ',
    download: 'ダウンロード',
    upload: 'アップロード',
    install: 'インストール',
    uninstall: 'アンインストール',
    enable: '有効',
    disable: '無効',
    restore: '復元',
    clear: 'クリア'
  },

  // ステータス
  status: {
    running: '実行中',
    stopped: '停止',
    starting: '起動中',
    stopping: '停止中',
    online: 'オンライン',
    offline: 'オフライン',
    connected: '接続済み',
    disconnected: '切断',
    enabled: '有効',
    disabled: '無効',
    active: 'アクティブ',
    inactive: '非アクティブ',
    pending: '保留中',
    processing: '処理中',
    completed: '完了',
    failed: '失敗',
    error: 'エラー',
    warning: '警告',
    success: '成功',
    loading: '読み込み中',
    saving: '保存中',
    deleting: '削除中',
    unknown: '不明',
    valid: '有効',
    invalid: '無効',
    expired: '期限切れ'
  },

  // メッセージ
  message: {
    confirmDelete: '削除してもよろしいですか？この操作は元に戻せません！',
    confirmBatchDelete: '選択した {count} 件を削除してもよろしいですか？この操作は元に戻せません！',
    deleteSuccess: '削除しました',
    deleteFailed: '削除に失敗しました',
    saveSuccess: '保存しました',
    saveFailed: '保存に失敗しました',
    createSuccess: '作成しました',
    createFailed: '作成に失敗しました',
    updateSuccess: '更新しました',
    updateFailed: '更新に失敗しました',
    copySuccess: 'クリップボードにコピーしました',
    copyFailed: 'コピーに失敗しました',
    operationSuccess: '操作が完了しました',
    operationFailed: '操作に失敗しました',
    loadingData: 'データを読み込み中...',
    noData: 'データがありません',
    noMoreData: 'これ以上データがありません',
    networkError: 'ネットワークエラーです。接続を確認してください',
    serverError: 'サーバーエラーです。後でもう一度お試しください',
    unknownError: '不明なエラー',
    inputRequired: '{field}を入力してください',
    selectRequired: '{field}を選択してください',
    invalidFormat: '{field}の形式が正しくありません',
    lengthLimit: '{field}は{max}文字以内で入力してください',
    confirmUnsaved: '保存されていない変更があります。本当に離れますか？'
  },

  // フォームラベル
  label: {
    name: '名前',
    title: 'タイトル',
    description: '説明',
    remark: '備考',
    type: 'タイプ',
    status: 'ステータス',
    sort: '並び順',
    createTime: '作成日時',
    updateTime: '更新日時',
    deleteTime: '削除日時',
    creator: '作成者',
    operator: '操作者',
    operation: '操作',
    enable: '有効化',
    required: '必須',
    optional: '任意'
  },

  // ページネーション
  pagination: {
    total: '全 {total} 件',
    pageSize: '{size} 件/ページ',
    goto: '移動',
    page: 'ページ',
    showing: '{from} から {to} 件を表示、全 {total} 件'
  },

  // 時間
  time: {
    today: '今日',
    yesterday: '昨日',
    lastWeek: '先週',
    lastMonth: '先月',
    justNow: 'たった今',
    minutesAgo: '{n}分前',
    hoursAgo: '{n}時間前',
    daysAgo: '{n}日前'
  },

  // 単位
  unit: {
    item: '個',
    piece: '件',
    times: '回',
    second: '秒',
    minute: '分',
    hour: '時間',
    day: '日',
    week: '週',
    month: '月',
    year: '年'
  },

  // 確認ダイアログ
  dialog: {
    confirmTitle: '確認',
    warningTitle: '警告',
    errorTitle: 'エラー',
    infoTitle: 'お知らせ',
    yes: 'はい',
    no: 'いいえ',
    ok: 'OK',
    cancel: 'キャンセル'
  },

  // 空の状態
  empty: {
    default: 'データがありません',
    search: '検索結果が見つかりません',
    filter: 'フィルター条件に一致するデータがありません'
  }
}
