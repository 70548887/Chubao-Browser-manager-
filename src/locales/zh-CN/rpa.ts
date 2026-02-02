/**
 * @description RPA机器人 - 中文
 */
export default {
  // 页面标题
  title: 'RPA 机器人',

  // 工具栏
  toolbar: {
    createFlow: '新建流程',
    importFlow: '导入流程',
    filterByStatus: '按状态筛选'
  },

  // 流程状态
  status: {
    draft: '草稿',
    ready: '就绪',
    running: '运行中',
    paused: '已暂停',
    completed: '已完成',
    failed: '失败',
    scheduled: '定时中'
  },

  // 表格列
  column: {
    name: '流程名称',
    description: '描述',
    status: '状态',
    lastRun: '最后运行',
    nextRun: '下次运行',
    runCount: '运行次数',
    successRate: '成功率',
    createTime: '创建时间',
    operation: '操作'
  },

  // 操作
  action: {
    run: '运行',
    pause: '暂停',
    stop: '停止',
    edit: '编辑',
    clone: '克隆',
    delete: '删除',
    viewLogs: '查看日志',
    schedule: '定时任务'
  },

  // 流程编辑器
  editor: {
    title: '流程编辑器',
    save: '保存流程',
    run: '运行流程',
    undo: '撤销',
    redo: '重做',
    zoomIn: '放大',
    zoomOut: '缩小',
    fitView: '适应视图',
    nodeLibrary: '节点库',
    properties: '属性面板'
  },

  // 节点类型
  node: {
    browser: '浏览器操作',
    navigation: '页面导航',
    click: '点击元素',
    input: '输入文本',
    extract: '提取数据',
    condition: '条件判断',
    loop: '循环',
    delay: '延时',
    screenshot: '截图',
    script: '自定义脚本'
  },

  // 空状态
  empty: {
    title: '暂无流程',
    description: '创建您的第一个自动化流程，提高工作效率',
    createButton: '新建流程'
  },

  // 消息提示
  message: {
    createSuccess: '流程创建成功',
    saveSuccess: '流程保存成功',
    runSuccess: '流程已开始运行',
    stopSuccess: '流程已停止',
    deleteConfirm: '确定要删除流程 "{name}" 吗？'
  }
}
