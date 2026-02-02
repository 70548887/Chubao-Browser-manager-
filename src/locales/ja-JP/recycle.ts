/**
 * @description ゴミ箱 - 日本語
 */
export default {
  // ページタイトル
  title: 'ゴミ箱',

  // ツールバー
  toolbar: {
    restore: '復元',
    permanentDelete: '完全削除',
    emptyRecycle: 'ゴミ箱を空にする',
    filterByType: 'タイプ別フィルター'
  },

  // アイテムタイプ
  type: {
    all: 'すべて',
    profile: 'ウィンドウ',
    group: 'グループ',
    proxy: 'プロキシ',
    tag: 'タグ',
    flow: 'フロー'
  },

  // テーブル列
  column: {
    name: '名前',
    type: 'タイプ',
    deleteTime: '削除日時',
    expireTime: '有効期限',
    deletedBy: '削除者',
    operation: '操作'
  },

  // 操作
  action: {
    restore: '復元',
    delete: '完全削除',
    preview: 'プレビュー'
  },

  // 空の状態
  empty: {
    title: 'ゴミ箱は空です',
    description: '削除されたアイテムは30日間ここに保存されます'
  },

  // メッセージ
  message: {
    restoreSuccess: '復元しました',
    restoreFailed: '復元に失敗しました',
    deleteSuccess: '完全に削除しました',
    deleteConfirm: '「{name}」を完全に削除しますか？この操作は元に戻せません！',
    emptyConfirm: 'ゴミ箱を空にしますか？すべてのアイテムが完全に削除されます！',
    emptySuccess: 'ゴミ箱を空にしました',
    expireTip: 'このアイテムは {time} 後に自動削除されます'
  },

  // ヒント
  tip: {
    autoDelete: 'ゴミ箱内のアイテムは削除後30日で自動的に完全削除されます',
    restoreHint: '復元後、アイテムは元の場所に戻ります'
  }
}
