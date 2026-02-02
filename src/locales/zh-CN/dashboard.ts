/**
 * @description 窗口列表/仪表盘 - 中文
 */
export default {
  // 页面标题
  title: '窗口列表',

  // 工具栏
  toolbar: {
    createWindow: '新建窗口',
    batchOpen: '批量打开',
    batchClose: '批量关闭',
    batchDelete: '批量删除',
    importProfile: '导入配置',
    exportProfile: '导出配置',
    sortBy: '排序',
    filterBy: '筛选',
    viewMode: '视图模式',
    listView: '列表视图',
    gridView: '网格视图'
  },

  // 排序选项
  sort: {
    lastUpdated: '最近更新',
    name: '名称',
    createTime: '创建时间',
    lastOpened: '最近打开'
  },

  // 筛选选项
  filter: {
    all: '全部',
    running: '运行中',
    stopped: '已停止',
    byGroup: '按分组',
    byTag: '按标签',
    byProxy: '按代理'
  },

  // 窗口卡片
  card: {
    id: 'ID',
    group: '分组',
    proxy: '代理',
    tags: '标签',
    lastOpened: '最后打开',
    createTime: '创建时间',
    noProxy: '直连',
    noGroup: '未分组',
    noTags: '无标签'
  },

  // 窗口操作
  action: {
    open: '打开窗口',
    close: '关闭窗口',
    edit: '编辑配置',
    clone: '克隆窗口',
    delete: '删除窗口',
    moveToGroup: '移动到分组',
    addTags: '添加标签',
    setProxy: '设置代理',
    viewDetail: '查看详情',
    copyId: '复制 ID'
  },

  // 新建窗口向导
  createWizard: {
    title: '新建窗口',
    step1: '基础设置',
    step2: '代理设置',
    step3: '指纹设置',
    step4: '偏好设置',
    basicInfo: '基础信息',
    windowName: '窗口名称',
    windowNamePlaceholder: '请输入窗口名称',
    selectGroup: '选择分组',
    selectTags: '选择标签',
    remark: '备注',
    remarkPlaceholder: '请输入备注信息（选填）'
  },

  // 代理设置
  proxySettings: {
    title: '代理设置',
    proxyType: '代理类型',
    directConnect: '直连（不使用代理）',
    selectProxy: '选择已有代理',
    createProxy: '创建新代理',
    noProxy: '暂无代理，请先创建'
  },

  // 指纹设置
  fingerprintSettings: {
    title: '指纹设置',
    autoGenerate: '自动生成',
    manualConfig: '手动配置',
    browserCore: '浏览器内核',
    userAgent: 'User Agent',
    screenResolution: '屏幕分辨率',
    language: '语言',
    timezone: '时区',
    webGL: 'WebGL',
    canvas: 'Canvas',
    audio: '音频指纹',
    fonts: '字体列表',
    plugins: '插件列表'
  },

  // 偏好设置
  preferenceSettings: {
    title: '偏好设置',
    startUrl: '启动页面',
    startUrlPlaceholder: '请输入启动时打开的URL',
    downloadPath: '下载路径',
    cookieSync: 'Cookie 同步',
    localStorage: 'LocalStorage',
    cache: '缓存策略'
  },

  // 空状态
  empty: {
    title: '暂无窗口',
    description: '点击上方按钮创建您的第一个浏览器窗口',
    createButton: '新建窗口'
  },

  // 批量操作
  batch: {
    selected: '已选择 {count} 个窗口',
    confirmDelete: '确定要删除选中的 {count} 个窗口吗？此操作不可恢复！',
    confirmClose: '确定要关闭选中的 {count} 个运行中的窗口吗？',
    openSuccess: '成功打开 {count} 个窗口',
    closeSuccess: '成功关闭 {count} 个窗口',
    deleteSuccess: '成功删除 {count} 个窗口'
  }
}
