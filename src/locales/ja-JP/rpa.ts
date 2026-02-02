/**
 * @description RPA ロボット - 日本語
 */
export default {
  // ページタイトル
  title: 'RPA ロボット',

  // ツールバー
  toolbar: {
    createFlow: 'フロー作成',
    importFlow: 'フローインポート',
    filterByStatus: 'ステータス別フィルター'
  },

  // フローステータス
  status: {
    draft: '下書き',
    ready: '準備完了',
    running: '実行中',
    paused: '一時停止',
    completed: '完了',
    failed: '失敗',
    scheduled: 'スケジュール中'
  },

  // テーブル列
  column: {
    name: 'フロー名',
    description: '説明',
    status: 'ステータス',
    lastRun: '最終実行',
    nextRun: '次回実行',
    runCount: '実行回数',
    successRate: '成功率',
    createTime: '作成日時',
    operation: '操作'
  },

  // 操作
  action: {
    run: '実行',
    pause: '一時停止',
    stop: '停止',
    edit: '編集',
    clone: '複製',
    delete: '削除',
    viewLogs: 'ログを見る',
    schedule: 'スケジュール'
  },

  // フローエディター
  editor: {
    title: 'フローエディター',
    save: 'フローを保存',
    run: 'フローを実行',
    undo: '元に戻す',
    redo: 'やり直す',
    zoomIn: '拡大',
    zoomOut: '縮小',
    fitView: 'ビューに合わせる',
    nodeLibrary: 'ノードライブラリ',
    properties: 'プロパティパネル'
  },

  // ノードタイプ
  node: {
    browser: 'ブラウザ操作',
    navigation: 'ページナビゲーション',
    click: 'クリック',
    input: 'テキスト入力',
    extract: 'データ抽出',
    condition: '条件分岐',
    loop: 'ループ',
    delay: '遅延',
    screenshot: 'スクリーンショット',
    script: 'カスタムスクリプト'
  },

  // 空の状態
  empty: {
    title: 'フローがありません',
    description: '最初の自動化フローを作成して、作業効率を向上させましょう',
    createButton: 'フロー作成'
  },

  // メッセージ
  message: {
    createSuccess: 'フローを作成しました',
    saveSuccess: 'フローを保存しました',
    runSuccess: 'フローの実行を開始しました',
    stopSuccess: 'フローを停止しました',
    deleteConfirm: 'フロー「{name}」を削除しますか？'
  }
}
