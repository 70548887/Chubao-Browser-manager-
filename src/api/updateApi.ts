/**
 * 应用更新 API
 * 
 * 核心原则：前端不直接请求外部服务器，所有调用走 Tauri invoke。
 */

import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

// ==================== 类型定义 ====================

export interface DownloadSource {
  id: number
  name: string
  url: string
  priority: number
  region: string
}

export interface UpdateInfo {
  has_update: boolean
  version?: string
  current_version: string
  latest_version?: string
  release_date?: string
  release_notes?: string
  mandatory?: boolean
  min_version?: string
  file_size?: number
  file_hash?: string
  downloads?: DownloadSource[]
}

export interface UpdateDownloadProgress {
  component: string
  downloaded: number
  total: number
  speed: number
  percent: number
  status: 'idle' | 'downloading' | 'verifying' | 'extracting' | 'completed' | 'failed'
  message: string
}

// ==================== 启动器更新 ====================

/** 检查启动器更新 */
export async function checkAppUpdate(): Promise<UpdateInfo> {
  return await invoke<UpdateInfo>('check_app_update')
}

/** 下载启动器更新（Rust 后端执行，含 SHA256 校验） */
export async function downloadAppUpdate(url: string, fileHash: string): Promise<string> {
  return await invoke<string>('download_app_update', { url, fileHash })
}

/** 安装启动器更新（优雅关闭 → 启动安装器 → 重启） */
export async function installAppUpdate(filePath: string): Promise<void> {
  await invoke('install_app_update', { filePath })
}

// ==================== 内核更新 ====================

/** 检查内核更新 */
export async function checkKernelUpdate(): Promise<UpdateInfo> {
  return await invoke<UpdateInfo>('check_kernel_update_api')
}

/** 下载内核更新 */
export async function downloadKernelUpdate(url: string, fileHash: string): Promise<string> {
  return await invoke<string>('download_kernel_update', { url, fileHash })
}

/** 安装内核更新（需先关闭所有浏览器） */
export async function installKernelUpdate(filePath: string, newVersion: string): Promise<void> {
  await invoke('install_kernel_update_cmd', { filePath, newVersion })
}

// ==================== 辅助 ====================

/** 获取运行中的浏览器数量 */
export async function getRunningBrowserCount(): Promise<number> {
  return await invoke<number>('get_running_browser_count')
}

/** 监听下载进度事件 */
export function onUpdateProgress(
  callback: (progress: UpdateDownloadProgress) => void
): Promise<UnlistenFn> {
  return listen<UpdateDownloadProgress>('update:download-progress', (event) => {
    callback(event.payload)
  })
}

/** 监听应用即将重启事件 */
export function onAppWillRestart(callback: () => void): Promise<UnlistenFn> {
  return listen('app:will-restart', () => callback())
}

/** 格式化文件大小 */
export function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

/** 格式化下载速度 */
export function formatSpeed(bytesPerSec: number): string {
  if (bytesPerSec < 1024) return `${bytesPerSec} B/s`
  if (bytesPerSec < 1024 * 1024) return `${(bytesPerSec / 1024).toFixed(1)} KB/s`
  return `${(bytesPerSec / (1024 * 1024)).toFixed(1)} MB/s`
}
