# IPC 接口说明（Tauri Commands）

本文档用于说明启动器（`browser-manager`）前端通过 Tauri IPC（`invoke`）调用 Rust 后端的接口列表。

- 前端调用方式：`import { invoke } from '@tauri-apps/api/core'`
- 命令注册位置：`src-tauri/src/lib.rs`

## 通用约定

### 1. 调用格式

```ts
await invoke('<command_name>', { /* args */ })
```

### 2. 参数命名大小写

Tauri 会按参数名映射 Rust command 的形参名，**参数名大小写必须严格一致**。

例如：
- `launch_browser` 需要 `{ profileId }`（不是 `profile_id`）
- `batch_launch_browsers` 需要 `{ profileIds }`

### 3. 返回与错误

- 成功：返回值为命令定义的返回类型。
- 失败：返回 `Promise.reject`，错误消息通常为 Rust 端的 `String`。

### 4. Profile 数据结构（后端返回）

后端 `Profile`（Rust）序列化后字段大致为：

- `id: string`
- `name: string`
- `group: string`
- `remark: string`
- `status: 'stopped' | 'running'`
- `fingerprint: object`
- `proxy: object | null`
- `created_at: string`（RFC3339，例如 `2026-01-27T14:10:00Z`）
- `updated_at: string`（RFC3339）

注意：如果前端 TypeScript 类型仍使用 `createdAt/updatedAt`（number）等字段，需要在前端做一次 DTO 适配/映射。

---

## 环境管理（Profile）

### 1) 获取环境列表

- **Command**：`get_profiles`
- **Args**：无
- **Return**：`Profile[]`

```ts
const list = await invoke('get_profiles')
```

### 2) 获取单个环境

- **Command**：`get_profile`
- **Args**：`{ id: string }`
- **Return**：`Profile`

```ts
const profile = await invoke('get_profile', { id })
```

### 3) 创建环境

- **Command**：`create_profile`
- **Args**：`{ data: CreateProfileDto }`
- **Return**：`Profile`

```ts
const created = await invoke('create_profile', {
  data: {
    name: 'test',
    group: 'default',
    remark: '',
    fingerprint: { /* ... */ },
    proxy: null,
  },
})
```

### 4) 更新环境

- **Command**：`update_profile`
- **Args**：`{ id: string, data: UpdateProfileDto }`
- **Return**：`Profile`

```ts
const updated = await invoke('update_profile', {
  id,
  data: { name: 'new name', remark: '...' },
})
```

### 5) 删除环境

- **Command**：`delete_profile`
- **Args**：`{ id: string }`
- **Return**：`void`

```ts
await invoke('delete_profile', { id })
```

### 6) 批量删除环境

- **Command**：`batch_delete_profiles`
- **Args**：`{ ids: string[] }`
- **Return**：`void`

```ts
await invoke('batch_delete_profiles', { ids })
```

---

## 系统设置（Settings）

Settings 数据来自 SQLite 的 `settings` 表，默认包含：
- `kernel_path`
- `user_data_dir`
- `default_proxy`

### 1) 获取单个设置

- **Command**：`get_setting_value`
- **Args**：`{ key: string }`
- **Return**：`string | null`

```ts
const kernelPath = await invoke<string | null>('get_setting_value', { key: 'kernel_path' })
```

### 2) 设置单个设置

- **Command**：`set_setting_value`
- **Args**：`{ key: string, value: string }`
- **Return**：`void`

```ts
await invoke('set_setting_value', { key: 'kernel_path', value: 'C:\\Path\\to\\chrome.exe' })
```

### 3) 获取全部设置

- **Command**：`get_all_settings`
- **Args**：无
- **Return**：`Record<string, string>`

```ts
const settings = await invoke<Record<string, string>>('get_all_settings')
```

---

## 浏览器控制（Browser）

### 前置条件

启动浏览器前必须保证：
- `kernel_path` 已设置为可执行内核路径
- `user_data_dir` 可选（为空则使用应用数据目录）

### 1) 启动浏览器

- **Command**：`launch_browser`
- **Args**：`{ profileId: string }`
- **Return**：`void`

```ts
await invoke('launch_browser', { profileId })
```

常见错误：
- `kernel_path is not set`

### 2) 停止浏览器

- **Command**：`stop_browser`
- **Args**：`{ profileId: string }`
- **Return**：`void`

```ts
await invoke('stop_browser', { profileId })
```

### 3) 批量启动

- **Command**：`batch_launch_browsers`
- **Args**：`{ profileIds: string[] }`
- **Return**：`void`

```ts
await invoke('batch_launch_browsers', { profileIds })
```

### 4) 批量停止

- **Command**：`batch_stop_browsers`
- **Args**：`{ profileIds: string[] }`
- **Return**：`void`

```ts
await invoke('batch_stop_browsers', { profileIds })
```

---

## 窗口控制（Window，Windows Only）

以下命令仅在 Windows 下可用；非 Windows 平台会返回错误 `windows only`。

### 1) 宫格排列窗口

- **Command**：`arrange_windows_grid`
- **Args**：`{ columns: number }`
- **Return**：`void`

```ts
await invoke('arrange_windows_grid', { columns: 3 })
```

### 2) 隐藏所有窗口（老板键）

- **Command**：`hide_all_windows`
- **Args**：无
- **Return**：`void`

```ts
await invoke('hide_all_windows')
```

### 3) 显示所有窗口

- **Command**：`show_all_windows`
- **Args**：无
- **Return**：`void`

```ts
await invoke('show_all_windows')
```

---

## 分组管理（Group）

### 1) 获取分组列表

- **Command**：`get_groups`
- **Args**：无
- **Return**：`Group[]`

```ts
const groups = await invoke('get_groups')
```

### 2) 获取单个分组

- **Command**：`get_group`
- **Args**：`{ id: string }`
- **Return**：`Group`

```ts
const group = await invoke('get_group', { id })
```

### 3) 创建分组

- **Command**：`create_group`
- **Args**：`{ data: CreateGroupDto }`
- **Return**：`Group`

```ts
const created = await invoke('create_group', {
  data: {
    name: '新分组',
    sort: 0,
    permission: 'all',
  },
})
```

### 4) 更新分组

- **Command**：`update_group`
- **Args**：`{ id: string, data: UpdateGroupDto }`
- **Return**：`Group`

```ts
const updated = await invoke('update_group', {
  id,
  data: { name: '新名称', sort: 1 },
})
```

### 5) 删除分组

- **Command**：`delete_group`
- **Args**：`{ id: string }`
- **Return**：`void`

```ts
await invoke('delete_group', { id })
```

注意：删除分组时，该分组下的窗口会自动移至默认分组。

---

## 标签管理（Tag）

### 1) 获取标签列表

- **Command**：`get_tags`
- **Args**：无
- **Return**：`Tag[]`

```ts
const tags = await invoke('get_tags')
```

### 2) 创建标签

- **Command**：`create_tag`
- **Args**：`{ data: CreateTagDto }`
- **Return**：`Tag`

```ts
const created = await invoke('create_tag', {
  data: {
    name: '重要',
    sort: 0,
  },
})
```

### 3) 更新标签

- **Command**：`update_tag`
- **Args**：`{ id: string, data: UpdateTagDto }`
- **Return**：`Tag`

```ts
const updated = await invoke('update_tag', {
  id,
  data: { name: '新标签名', sort: 1 },
})
```

### 4) 删除标签

- **Command**：`delete_tag`
- **Args**：`{ id: string }`
- **Return**：`void`

```ts
await invoke('delete_tag', { id })
```

---

## 代理管理（Proxy）

### 1) 获取代理列表

- **Command**：`get_proxies`
- **Args**：无
- **Return**：`Proxy[]`

```ts
const proxies = await invoke('get_proxies')
```

### 2) 获取单个代理

- **Command**：`get_proxy`
- **Args**：`{ id: string }`
- **Return**：`Proxy`

```ts
const proxy = await invoke('get_proxy', { id })
```

### 3) 创建代理

- **Command**：`create_proxy`
- **Args**：`{ data: CreateProxyDto }`
- **Return**：`Proxy`

```ts
const created = await invoke('create_proxy', {
  data: {
    type: 'http',
    host: '127.0.0.1',
    port: '8080',
    username: '',
    password: '',
    tag: '',
    remark: '',
  },
})
```

### 4) 更新代理

- **Command**：`update_proxy`
- **Args**：`{ id: string, data: UpdateProxyDto }`
- **Return**：`Proxy`

```ts
const updated = await invoke('update_proxy', {
  id,
  data: { host: '192.168.1.1', port: '8888' },
})
```

### 5) 删除代理

- **Command**：`delete_proxy`
- **Args**：`{ id: string }`
- **Return**：`void`

```ts
await invoke('delete_proxy', { id })
```

### 6) 检测代理

- **Command**：`test_proxy`
- **Args**：`{ id: string }`
- **Return**：`ProxyTestResult`

```ts
const result = await invoke('test_proxy', { id })
// result: { success: boolean, ip?: string, location?: string, latency?: number, error?: string }
```

---

## 回收站管理（Recycle Bin）

### 1) 获取回收站列表

- **Command**：`get_recycle_bin`
- **Args**：无
- **Return**：`RecycledProfile[]`

```ts
const recycled = await invoke('get_recycle_bin')
```

### 2) 恢复窗口

- **Command**：`restore_profile`
- **Args**：`{ id: string }`
- **Return**：`void`

```ts
await invoke('restore_profile', { id })
```

### 3) 批量恢复窗口

- **Command**：`batch_restore_profiles`
- **Args**：`{ ids: string[] }`
- **Return**：`void`

```ts
await invoke('batch_restore_profiles', { ids })
```

### 4) 永久删除窗口

- **Command**：`permanently_delete_profile`
- **Args**：`{ id: string }`
- **Return**：`void`

```ts
await invoke('permanently_delete_profile', { id })
```

### 5) 批量永久删除

- **Command**：`batch_permanently_delete_profiles`
- **Args**：`{ ids: string[] }`
- **Return**：`void`

```ts
await invoke('batch_permanently_delete_profiles', { ids })
```

### 6) 清空回收站

- **Command**：`empty_recycle_bin`
- **Args**：无
- **Return**：`void`

```ts
await invoke('empty_recycle_bin')
```

---

## 数据类型定义（TypeScript）

### Group

```ts
interface Group {
  id: string
  name: string
  sort: number
  profileCount: number
  permission: string
  createdAt: string  // RFC3339
  updatedAt: string  // RFC3339
}
```

### Tag

```ts
interface Tag {
  id: string
  name: string
  sort: number
  windowCount: number
  createdAt: string  // RFC3339
}
```

### Proxy

```ts
interface Proxy {
  id: string
  type: 'http' | 'https' | 'socks5' | 'direct'
  source: string
  tag: string
  host: string
  port: string
  username?: string
  password?: string
  ipAddress?: string
  location?: string
  usedCount: number
  autoCheck: boolean
  expireAt?: string  // RFC3339
  bindWindow?: string
  remark: string
  status: 'active' | 'expired' | 'error'
  createdAt: string  // RFC3339
}
```

### RecycledProfile

```ts
interface RecycledProfile extends Profile {
  deletedAt: string  // RFC3339
  deletedBy?: string
}
```

---

## 注意事项

1. **日期时间格式**：后端返回的所有时间字段均为 RFC3339 格式的字符串（如 `2026-01-27T14:10:00Z`），前端需要转换为 `Date` 或时间戳。

2. **错误处理**：所有命令失败时都会返回 `Promise.reject`，建议使用 `try-catch` 包裹。

3. **权限控制**：部分操作可能需要检查分组权限，后端会返回相应错误。

4. **回收站自动清理**：回收站中的数据会在 30 天后自动永久删除。

5. **批量操作**：批量操作失败时，部分项可能已成功，建议操作后刷新列表。
