/**
 * @description 分组管理 - 中文
 */
export default {
  // 页面标题
  title: '分组管理',

  // 工具栏
  toolbar: {
    addGroup: '新建分组',
    batchDelete: '批量删除',
    sortBy: '排序'
  },

  // 排序选项
  sort: {
    lastUpdated: '最近更新',
    name: '按名称',
    sortValue: '按排序值'
  },

  // 表格列
  column: {
    index: '序号',
    name: '分组名称',
    profileCount: '窗口数量',
    sort: '排序',
    remark: '备注',
    createTime: '创建时间',
    operation: '操作'
  },

  // 添加/编辑表单
  form: {
    addTitle: '新建分组',
    editTitle: '编辑分组',
    name: '分组名称',
    namePlaceholder: '请输入分组名称',
    icon: '图标',
    remark: '备注信息',
    remarkPlaceholder: '可选填备注信息',
    sort: '排序',
    sortTip: '数值越小排序越靠前'
  },

  // 图标选项
  icon: {
    folder: '文件夹',
    shopping: '购物',
    campaign: '营销',
    movie: '影视',
    payments: '支付',
    mail: '邮件'
  },

  // 详情抽屉
  detail: {
    title: '分组详情',
    basicInfo: '基本信息',
    statistics: '资源统计',
    relatedProfiles: '关联窗口',
    activeSessions: '活跃会话',
    quickActions: '快捷操作',
    batchProxy: '批量配置代理',
    manageExtensions: '管理扩展插件',
    shareToTeam: '分享给团队成员'
  },

  // 空状态
  empty: {
    title: '暂无分组数据',
    createButton: '新建分组'
  },

  // 消息提示
  message: {
    createSuccess: '分组创建成功',
    updateSuccess: '分组更新成功',
    deleteSuccess: '分组删除成功',
    deleteConfirm: '确定要删除分组 "{name}" 吗？',
    defaultGroupTip: '默认分组不可删除',
    hasProfiles: '该分组内还有 {count} 个窗口，请先移动或删除这些窗口',
    batchDeleteConfirm: '确定要删除选中的 {count} 个分组吗？此操作不可恢复！'
  },

  // 创建提示
  createTip: '创建分组后，您可以将现有的浏览器环境移动到该分组中，或在该分组下直接创建新环境。'
}
