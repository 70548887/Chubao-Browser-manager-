/**
 * @description 全局消息提示服务 - 统一配置显示位置在自定义标题栏下方
 */
import { ElMessage, type MessageOptions, type MessageHandler } from 'element-plus'

// 自定义标题栏高度 40px + 间距 16px = 56px
const MESSAGE_OFFSET = 56

type MessageType = 'success' | 'warning' | 'info' | 'error'
type MessageParams = string | MessageOptions

/**
 * 创建带默认 offset 的消息配置
 */
const createOptions = (params: MessageParams, type?: MessageType): MessageOptions => {
  if (typeof params === 'string') {
    return {
      message: params,
      type,
      offset: MESSAGE_OFFSET,
    }
  }
  return {
    ...params,
    offset: params.offset ?? MESSAGE_OFFSET,
  }
}

/**
 * 成功消息
 */
function success(params: MessageParams): MessageHandler {
  return ElMessage(createOptions(params, 'success'))
}

/**
 * 错误消息
 */
function error(params: MessageParams): MessageHandler {
  return ElMessage(createOptions(params, 'error'))
}

/**
 * 警告消息
 */
function warning(params: MessageParams): MessageHandler {
  return ElMessage(createOptions(params, 'warning'))
}

/**
 * 信息消息
 */
function info(params: MessageParams): MessageHandler {
  return ElMessage(createOptions(params, 'info'))
}

/**
 * 通用消息（可自定义类型）
 */
function show(options: MessageOptions): MessageHandler {
  return ElMessage(createOptions(options))
}

/**
 * 关闭所有消息
 */
function closeAll(): void {
  ElMessage.closeAll()
}

// 导出封装后的 Message 对象，可作为 ElMessage 的直接替代品
export const Message = {
  success,
  error,
  warning,
  info,
  show,
  closeAll,
}

// 同时导出单独的函数
export { success as showSuccess, error as showError, warning as showWarning, info as showInfo }

export default Message
