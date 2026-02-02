/**
 * @description 通用词汇 - 中文
 */
export default {
  // 操作按钮
  action: {
    add: '新建',
    create: '创建',
    edit: '编辑',
    delete: '删除',
    remove: '移除',
    save: '保存',
    cancel: '取消',
    confirm: '确定',
    close: '关闭',
    search: '搜索',
    filter: '筛选',
    sort: '排序',
    refresh: '刷新',
    reset: '重置',
    export: '导出',
    import: '导入',
    copy: '复制',
    paste: '粘贴',
    selectAll: '全选',
    batchDelete: '批量删除',
    batchOpen: '批量打开',
    batchClose: '批量关闭',
    start: '启动',
    stop: '停止',
    open: '打开',
    view: '查看',
    detail: '详情',
    back: '返回',
    next: '下一步',
    prev: '上一步',
    submit: '提交',
    apply: '应用',
    test: '测试',
    check: '检测',
    retry: '重试',
    more: '更多',
    expand: '展开',
    collapse: '收起',
    download: '下载',
    upload: '上传',
    install: '安装',
    uninstall: '卸载',
    enable: '启用',
    disable: '禁用',
    restore: '恢复',
    clear: '清空'
  },

  // 状态
  status: {
    running: '运行中',
    stopped: '已停止',
    starting: '启动中',
    stopping: '停止中',
    online: '在线',
    offline: '离线',
    connected: '已连接',
    disconnected: '已断开',
    enabled: '已启用',
    disabled: '已禁用',
    active: '活跃',
    inactive: '不活跃',
    pending: '待处理',
    processing: '处理中',
    completed: '已完成',
    failed: '失败',
    error: '错误',
    warning: '警告',
    success: '成功',
    loading: '加载中',
    saving: '保存中',
    deleting: '删除中',
    unknown: '未知',
    valid: '有效',
    invalid: '无效',
    expired: '已过期'
  },

  // 提示消息
  message: {
    confirmDelete: '确定要删除吗？此操作不可恢复！',
    confirmBatchDelete: '确定要批量删除选中的 {count} 项吗？此操作不可恢复！',
    deleteSuccess: '删除成功',
    deleteFailed: '删除失败',
    saveSuccess: '保存成功',
    saveFailed: '保存失败',
    createSuccess: '创建成功',
    createFailed: '创建失败',
    updateSuccess: '更新成功',
    updateFailed: '更新失败',
    copySuccess: '已复制到剪切板',
    copyFailed: '复制失败',
    operationSuccess: '操作成功',
    operationFailed: '操作失败',
    loadingData: '正在加载数据...',
    noData: '暂无数据',
    noMoreData: '没有更多数据了',
    networkError: '网络错误，请检查网络连接',
    serverError: '服务器错误，请稍后重试',
    unknownError: '未知错误',
    inputRequired: '请输入{field}',
    selectRequired: '请选择{field}',
    invalidFormat: '{field}格式不正确',
    lengthLimit: '{field}长度不能超过{max}个字符',
    confirmUnsaved: '有未保存的更改，确定要离开吗？'
  },

  // 表单标签
  label: {
    name: '名称',
    title: '标题',
    description: '描述',
    remark: '备注',
    type: '类型',
    status: '状态',
    sort: '排序',
    createTime: '创建时间',
    updateTime: '更新时间',
    deleteTime: '删除时间',
    creator: '创建者',
    operator: '操作人',
    operation: '操作',
    enable: '是否启用',
    required: '必填',
    optional: '选填'
  },

  // 分页
  pagination: {
    total: '共 {total} 条',
    pageSize: '{size} 条/页',
    goto: '跳至',
    page: '页',
    showing: '显示 {from} 到 {to} 条，共 {total} 条'
  },

  // 时间
  time: {
    today: '今天',
    yesterday: '昨天',
    lastWeek: '上周',
    lastMonth: '上月',
    justNow: '刚刚',
    minutesAgo: '{n}分钟前',
    hoursAgo: '{n}小时前',
    daysAgo: '{n}天前'
  },

  // 单位
  unit: {
    item: '个',
    piece: '条',
    times: '次',
    second: '秒',
    minute: '分钟',
    hour: '小时',
    day: '天',
    week: '周',
    month: '月',
    year: '年'
  },

  // 确认对话框
  dialog: {
    confirmTitle: '确认',
    warningTitle: '警告',
    errorTitle: '错误',
    infoTitle: '提示',
    yes: '是',
    no: '否',
    ok: '确定',
    cancel: '取消'
  },

  // 空状态
  empty: {
    default: '暂无数据',
    search: '未找到匹配结果',
    filter: '没有符合筛选条件的数据'
  }
}
