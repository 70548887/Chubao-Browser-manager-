/**
 * @description 代理管理 - 中文
 */
export default {
  // 页面标题
  title: '代理管理',

  // 工具栏
  toolbar: {
    addProxy: '添加代理',
    batchImport: '批量导入',
    batchExport: '批量导出',
    batchDelete: '批量删除',
    batchCheck: '批量检测',
    filterByType: '按类型筛选',
    filterByStatus: '按状态筛选'
  },

  // 代理类型
  type: {
    http: 'HTTP',
    https: 'HTTPS',
    socks4: 'SOCKS4',
    socks5: 'SOCKS5',
    ssh: 'SSH'
  },

  // 代理状态
  status: {
    valid: '有效',
    invalid: '无效',
    checking: '检测中',
    unchecked: '未检测',
    expired: '已过期'
  },

  // 表格列
  column: {
    name: '代理名称',
    type: '类型',
    host: '主机地址',
    port: '端口',
    username: '用户名',
    password: '密码',
    status: '状态',
    delay: '延迟',
    location: '位置',
    usedBy: '使用数量',
    expireTime: '过期时间',
    lastCheck: '最后检测',
    createTime: '创建时间',
    operation: '操作'
  },

  // 添加/编辑表单
  form: {
    title: '代理信息',
    addTitle: '添加代理',
    editTitle: '编辑代理',
    name: '代理名称',
    namePlaceholder: '请输入代理名称',
    type: '代理类型',
    host: '主机地址',
    hostPlaceholder: '请输入主机地址或IP',
    port: '端口号',
    portPlaceholder: '请输入端口号',
    username: '用户名',
    usernamePlaceholder: '请输入用户名（选填）',
    password: '密码',
    passwordPlaceholder: '请输入密码（选填）',
    remark: '备注',
    remarkPlaceholder: '请输入备注信息'
  },

  // 批量导入
  import: {
    title: '批量导入代理',
    description: '支持以下格式导入：',
    format1: 'host:port',
    format2: 'host:port:username:password',
    format3: 'type://host:port',
    format4: 'type://username:password@host:port',
    placeholder: '请输入代理信息，每行一个',
    parseResult: '解析结果：成功 {success} 个，失败 {failed} 个',
    importButton: '开始导入'
  },

  // 操作
  action: {
    check: '检测',
    edit: '编辑',
    delete: '删除',
    copyInfo: '复制信息'
  },

  // 检测结果
  checkResult: {
    success: '连接成功',
    failed: '连接失败',
    timeout: '连接超时',
    delay: '延迟 {ms}ms',
    location: '位置：{location}'
  },

  // 空状态
  empty: {
    title: '暂无代理',
    description: '点击上方按钮添加您的第一个代理',
    addButton: '添加代理'
  },

  // 消息提示
  message: {
    checkSuccess: '代理检测成功，延迟 {ms}ms',
    checkFailed: '代理检测失败：{reason}',
    deleteConfirm: '确定要删除代理 "{name}" 吗？',
    deleteWithProfiles: '该代理正被 {count} 个窗口使用，删除后这些窗口将变为直连模式。确定要删除吗？',
    importSuccess: '成功导入 {count} 个代理',
    exportSuccess: '成功导出 {count} 个代理'
  }
}
