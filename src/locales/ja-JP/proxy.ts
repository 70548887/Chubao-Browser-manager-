/**
 * @description プロキシ管理 - 日本語
 */
export default {
  // ページタイトル
  title: 'プロキシ管理',

  // ツールバー
  toolbar: {
    addProxy: 'プロキシを追加',
    batchImport: '一括インポート',
    batchExport: '一括エクスポート',
    batchDelete: '一括削除',
    batchCheck: '一括検出',
    filterByType: 'タイプ別フィルター',
    filterByStatus: 'ステータス別フィルター'
  },

  // プロキシタイプ
  type: {
    http: 'HTTP',
    https: 'HTTPS',
    socks4: 'SOCKS4',
    socks5: 'SOCKS5',
    ssh: 'SSH'
  },

  // プロキシステータス
  status: {
    valid: '有効',
    invalid: '無効',
    checking: '検出中',
    unchecked: '未検出',
    expired: '期限切れ'
  },

  // テーブル列
  column: {
    name: 'プロキシ名',
    type: 'タイプ',
    host: 'ホストアドレス',
    port: 'ポート',
    username: 'ユーザー名',
    password: 'パスワード',
    status: 'ステータス',
    delay: '遅延',
    location: '場所',
    usedBy: '使用数',
    expireTime: '有効期限',
    lastCheck: '最終検出',
    createTime: '作成日時',
    operation: '操作'
  },

  // 追加/編集フォーム
  form: {
    title: 'プロキシ情報',
    addTitle: 'プロキシを追加',
    editTitle: 'プロキシを編集',
    name: 'プロキシ名',
    namePlaceholder: 'プロキシ名を入力',
    type: 'プロキシタイプ',
    host: 'ホストアドレス',
    hostPlaceholder: 'ホストアドレスまたはIPを入力',
    port: 'ポート番号',
    portPlaceholder: 'ポート番号を入力',
    username: 'ユーザー名',
    usernamePlaceholder: 'ユーザー名を入力（任意）',
    password: 'パスワード',
    passwordPlaceholder: 'パスワードを入力（任意）',
    remark: '備考',
    remarkPlaceholder: '備考を入力'
  },

  // 一括インポート
  import: {
    title: 'プロキシを一括インポート',
    description: '以下の形式でインポートできます：',
    format1: 'host:port',
    format2: 'host:port:username:password',
    format3: 'type://host:port',
    format4: 'type://username:password@host:port',
    placeholder: 'プロキシ情報を入力（1行に1つ）',
    parseResult: '解析結果：成功 {success} 件、失敗 {failed} 件',
    importButton: 'インポート開始'
  },

  // 操作
  action: {
    check: '検出',
    edit: '編集',
    delete: '削除',
    copyInfo: '情報をコピー'
  },

  // 検出結果
  checkResult: {
    success: '接続成功',
    failed: '接続失敗',
    timeout: '接続タイムアウト',
    delay: '遅延 {ms}ms',
    location: '場所：{location}'
  },

  // 空の状態
  empty: {
    title: 'プロキシがありません',
    description: '上のボタンをクリックして最初のプロキシを追加',
    addButton: 'プロキシを追加'
  },

  // メッセージ
  message: {
    checkSuccess: 'プロキシ検出成功、遅延 {ms}ms',
    checkFailed: 'プロキシ検出失敗：{reason}',
    deleteConfirm: 'プロキシ「{name}」を削除しますか？',
    deleteWithProfiles: 'このプロキシは {count} 個のウィンドウで使用中です。削除後、これらのウィンドウは直接接続になります。削除しますか？',
    importSuccess: '{count} 件のプロキシをインポートしました',
    exportSuccess: '{count} 件のプロキシをエクスポートしました'
  }
}
