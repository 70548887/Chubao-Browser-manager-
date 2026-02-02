/**
 * @description タグ管理 - 日本語
 */
export default {
  // ページタイトル
  title: 'タグ管理',

  // ツールバー
  toolbar: {
    addTag: 'タグを追加',
    batchDelete: '一括削除'
  },

  // テーブル列
  column: {
    name: 'タグ名',
    color: '色',
    profileCount: 'ウィンドウ数',
    createTime: '作成日時',
    operation: '操作'
  },

  // 追加/編集フォーム
  form: {
    addTitle: 'タグを追加',
    editTitle: 'タグを編集',
    name: 'タグ名',
    namePlaceholder: 'タグ名を入力',
    color: 'タグの色',
    selectColor: '色を選択'
  },

  // 色オプション
  color: {
    blue: '青',
    green: '緑',
    orange: 'オレンジ',
    red: '赤',
    purple: '紫',
    pink: 'ピンク',
    cyan: 'シアン',
    gray: 'グレー'
  },

  // 空の状態
  empty: {
    title: 'タグがありません',
    description: 'タグを使ってウィンドウをより効率的に管理・フィルターできます',
    addButton: 'タグを追加'
  },

  // メッセージ
  message: {
    createSuccess: 'タグを作成しました',
    updateSuccess: 'タグを更新しました',
    deleteSuccess: 'タグを削除しました',
    deleteConfirm: 'タグ「{name}」を削除しますか？',
    hasProfiles: 'このタグは {count} 個のウィンドウに関連付けられています。削除後、これらのウィンドウからこのタグが削除されます'
  }
}
