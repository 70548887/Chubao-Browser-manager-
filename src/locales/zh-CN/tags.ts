/**
 * @description 标签管理 - 中文
 */
export default {
  // 页面标题
  title: '标签管理',

  // 工具栏
  toolbar: {
    addTag: '添加标签',
    batchDelete: '批量删除'
  },

  // 表格列
  column: {
    name: '标签名称',
    color: '颜色',
    profileCount: '窗口数量',
    createTime: '创建时间',
    operation: '操作'
  },

  // 添加/编辑表单
  form: {
    addTitle: '添加标签',
    editTitle: '编辑标签',
    name: '标签名称',
    namePlaceholder: '请输入标签名称',
    color: '标签颜色',
    selectColor: '选择颜色'
  },

  // 颜色选项
  color: {
    blue: '蓝色',
    green: '绿色',
    orange: '橙色',
    red: '红色',
    purple: '紫色',
    pink: '粉色',
    cyan: '青色',
    gray: '灰色'
  },

  // 空状态
  empty: {
    title: '暂无标签',
    description: '标签可以帮助您更好地管理和筛选窗口',
    addButton: '添加标签'
  },

  // 消息提示
  message: {
    createSuccess: '标签创建成功',
    updateSuccess: '标签更新成功',
    deleteSuccess: '标签删除成功',
    deleteConfirm: '确定要删除标签 "{name}" 吗？',
    hasProfiles: '该标签已关联 {count} 个窗口，删除后这些窗口将移除此标签'
  }
}
