/**
 * @description グループ管理 - 日本語
 */
export default {
  // ページタイトル
  title: 'グループ管理',

  // ツールバー
  toolbar: {
    addGroup: 'グループ作成',
    batchDelete: '一括削除',
    sortBy: '並び替え'
  },

  // 並び替えオプション
  sort: {
    lastUpdated: '最近更新',
    name: '名前順',
    sortValue: '並び順'
  },

  // テーブル列
  column: {
    index: '番号',
    name: 'グループ名',
    profileCount: 'ウィンドウ数',
    sort: '並び順',
    remark: '備考',
    createTime: '作成日時',
    operation: '操作'
  },

  // 追加/編集フォーム
  form: {
    addTitle: 'グループ作成',
    editTitle: 'グループ編集',
    name: 'グループ名',
    namePlaceholder: 'グループ名を入力',
    icon: 'アイコン',
    remark: '備考',
    remarkPlaceholder: '備考を入力（任意）',
    sort: '並び順',
    sortTip: '値が小さいほど上に表示'
  },

  // アイコンオプション
  icon: {
    folder: 'フォルダ',
    shopping: 'ショッピング',
    campaign: 'マーケティング',
    movie: '動画',
    payments: '決済',
    mail: 'メール'
  },

  // 詳細ドロワー
  detail: {
    title: 'グループ詳細',
    basicInfo: '基本情報',
    statistics: 'リソース統計',
    relatedProfiles: '関連ウィンドウ',
    activeSessions: 'アクティブセッション',
    quickActions: 'クイック操作',
    batchProxy: 'プロキシ一括設定',
    manageExtensions: '拡張機能管理',
    shareToTeam: 'チームに共有'
  },

  // 空の状態
  empty: {
    title: 'グループがありません',
    createButton: 'グループ作成'
  },

  // メッセージ
  message: {
    createSuccess: 'グループを作成しました',
    updateSuccess: 'グループを更新しました',
    deleteSuccess: 'グループを削除しました',
    deleteConfirm: 'グループ「{name}」を削除しますか？',
    defaultGroupTip: 'デフォルトグループは削除できません',
    hasProfiles: 'このグループには {count} 個のウィンドウがあります。先にウィンドウを移動または削除してください',
    batchDeleteConfirm: '選択した {count} 個のグループを削除しますか？この操作は元に戻せません！'
  },

  // 作成ヒント
  createTip: 'グループ作成後、既存のブラウザ環境をこのグループに移動したり、このグループ内で新しい環境を作成できます。'
}
