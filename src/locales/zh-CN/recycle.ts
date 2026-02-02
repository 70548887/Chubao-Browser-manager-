/**
 * @description 回收站 - 中文
 */
export default {
  // 页面标题
  title: '回收站',

  // 工具栏
  toolbar: {
    restore: '恢复',
    permanentDelete: '永久删除',
    emptyRecycle: '清空回收站',
    filterByType: '按类型筛选'
  },

  // 项目类型
  type: {
    all: '全部',
    profile: '窗口',
    group: '分组',
    proxy: '代理',
    tag: '标签',
    flow: '流程'
  },

  // 表格列
  column: {
    name: '名称',
    type: '类型',
    deleteTime: '删除时间',
    expireTime: '过期时间',
    deletedBy: '删除者',
    operation: '操作'
  },

  // 操作
  action: {
    restore: '恢复',
    delete: '永久删除',
    preview: '预览'
  },

  // 空状态
  empty: {
    title: '回收站为空',
    description: '删除的项目会在这里保留30天'
  },

  // 消息提示
  message: {
    restoreSuccess: '恢复成功',
    restoreFailed: '恢复失败',
    deleteSuccess: '永久删除成功',
    deleteConfirm: '确定要永久删除 "{name}" 吗？此操作不可恢复！',
    emptyConfirm: '确定要清空回收站吗？所有项目将被永久删除！',
    emptySuccess: '回收站已清空',
    expireTip: '该项目将于 {time} 后自动删除'
  },

  // 提示信息
  tip: {
    autoDelete: '回收站内的项目将在删除30天后自动永久删除',
    restoreHint: '恢复后的项目将回到原来的位置'
  }
}
