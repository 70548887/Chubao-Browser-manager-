# API 配置使用指南

## 配置文件位置

**主配置文件:** `src/config/api.config.ts`

此文件集中管理所有后台 API 接口地址，包括：
- 用户认证
- 应用更新
- 内核下载
- 消息通知
- 许可证验证
- 数据同步
- 统计上报
- 反馈支持

---

## 环境配置

### 支持的环境

| 环境 | API 地址 | 说明 |
|------|---------|------|
| `development` | `http://localhost:8080` | 本地开发环境 |
| `staging` | `https://api-staging.qutab.cn` | 测试环境 |
| `production` | `https://api.qutab.cn` | 生产环境 |

### 切换环境

**方式 1: 通过 package.json 脚本**

```json
{
  "scripts": {
    "dev": "vite --mode development",
    "build:staging": "vite build --mode staging",
    "build:prod": "vite build --mode production"
  }
}
```

**方式 2: 通过环境变量文件**

创建 `.env.local` 文件（此文件不会被提交到 Git）:

```env
VITE_API_BASE_URL=http://localhost:8080
```

---

## 使用示例

### 1. 基础使用

```typescript
import { ApiEndpoints, buildApiUrl } from '@/config'

// 直接使用端点
const loginUrl = ApiEndpoints.AUTH.LOGIN
console.log(loginUrl) // http://localhost:8080/api/v1/auth/login

// 带查询参数
const url = buildApiUrl(ApiEndpoints.KERNEL.DOWNLOAD_INFO, {
  platform: 'windows',
  arch: 'x86_64',
  launcher_version: '0.3.0'
})
// 结果: http://localhost:8080/api/v1/kernel/download-info?platform=windows&arch=x86_64&launcher_version=0.3.0
```

### 2. 在 API 层使用

```typescript
// src/api/authApi.ts
import { ApiEndpoints } from '@/config'

export async function login(username: string, password: string) {
  const response = await fetch(ApiEndpoints.AUTH.LOGIN, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ username, password })
  })
  return response.json()
}

export async function register(data: RegisterData) {
  const response = await fetch(ApiEndpoints.AUTH.REGISTER, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data)
  })
  return response.json()
}
```

### 3. 在组件中使用

```typescript
// 某个 Vue 组件
import { ApiEndpoints, buildApiUrl } from '@/config'

const checkForUpdates = async () => {
  const url = buildApiUrl(ApiEndpoints.UPDATE.CHECK_LAUNCHER, {
    version: '0.3.0',
    platform: 'windows',
    arch: 'x86_64'
  })
  
  const response = await fetch(url)
  const result = await response.json()
  
  if (result.code === 0 && result.data.has_update) {
    console.log('发现新版本:', result.data.version)
  }
}
```

### 4. WebSocket 连接

```typescript
import { getWebSocketUrl, ApiEndpoints } from '@/config'

const ws = new WebSocket(ApiEndpoints.NOTIFICATION.WEBSOCKET)

ws.onmessage = (event) => {
  const message = JSON.parse(event.data)
  console.log('收到通知:', message)
}
```

---

## API 端点清单

### 用户认证 (AUTH)

| 接口 | 路径 | 方法 | 说明 |
|------|------|------|------|
| 登录 | `/api/v1/auth/login` | POST | 用户登录 |
| 注册 | `/api/v1/auth/register` | POST | 用户注册 |
| 登出 | `/api/v1/auth/logout` | POST | 用户登出 |
| 刷新Token | `/api/v1/auth/refresh` | POST | 刷新访问令牌 |
| 用户信息 | `/api/v1/auth/user` | GET | 获取当前用户信息 |

### 应用更新 (UPDATE)

| 接口 | 路径 | 方法 | 说明 |
|------|------|------|------|
| 检查启动器更新 | `/api/v1/updates/launcher` | GET | 检查启动器是否有新版本 |
| 检查内核更新 | `/api/v1/updates/kernel` | GET | 检查 Chromium 内核更新 |
| 获取更新日志 | `/api/v1/updates/changelog` | GET | 获取版本更新日志 |

### 内核下载 (KERNEL)

| 接口 | 路径 | 方法 | 说明 |
|------|------|------|------|
| 获取下载信息 | `/api/v1/kernel/download-info` | GET | 获取内核下载地址和信息 |
| 版本列表 | `/api/v1/kernel/versions` | GET | 获取可用内核版本列表 |
| 上报安装状态 | `/api/v1/kernel/install-report` | POST | 上报内核安装成功/失败 |

### 消息通知 (NOTIFICATION)

| 接口 | 路径 | 方法 | 说明 |
|------|------|------|------|
| 消息列表 | `/api/v1/notifications` | GET | 获取通知消息列表 |
| 标记已读 | `/api/v1/notifications/read` | PUT | 标记消息为已读 |
| 未读数量 | `/api/v1/notifications/unread-count` | GET | 获取未读消息数量 |
| WebSocket | `/ws/notifications` | WS | 实时消息推送 |

---

## 请求配置

### 全局配置

```typescript
import { ApiConfig } from '@/config'

console.log(ApiConfig.TIMEOUT)  // 30000ms
console.log(ApiConfig.RETRY.MAX_ATTEMPTS)  // 3
console.log(ApiConfig.HEADERS)  // 默认请求头
```

### 带重试的请求封装

```typescript
import { ApiConfig } from '@/config'

async function fetchWithRetry(url: string, options: RequestInit = {}, retries = 0): Promise<Response> {
  try {
    const response = await fetch(url, {
      ...options,
      headers: {
        ...ApiConfig.HEADERS,
        ...options.headers,
      },
      signal: AbortSignal.timeout(ApiConfig.TIMEOUT),
    })

    // 检查是否需要重试
    if (ApiConfig.RETRY.RETRY_STATUS_CODES.includes(response.status) && 
        retries < ApiConfig.RETRY.MAX_ATTEMPTS) {
      await new Promise(resolve => setTimeout(resolve, ApiConfig.RETRY.DELAY))
      return fetchWithRetry(url, options, retries + 1)
    }

    return response
  } catch (error) {
    if (retries < ApiConfig.RETRY.MAX_ATTEMPTS) {
      await new Promise(resolve => setTimeout(resolve, ApiConfig.RETRY.DELAY))
      return fetchWithRetry(url, options, retries + 1)
    }
    throw error
  }
}
```

---

## 最佳实践

### 1. 创建统一的 HTTP 客户端

```typescript
// src/utils/httpClient.ts
import { ApiConfig, API_BASE_URL } from '@/config'

class HttpClient {
  private baseURL: string

  constructor(baseURL: string) {
    this.baseURL = baseURL
  }

  async request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
    const url = endpoint.startsWith('http') ? endpoint : `${this.baseURL}${endpoint}`
    
    const response = await fetch(url, {
      ...options,
      headers: {
        ...ApiConfig.HEADERS,
        ...options.headers,
      },
    })

    if (!response.ok) {
      throw new Error(`HTTP Error: ${response.status}`)
    }

    return response.json()
  }

  get<T>(endpoint: string, params?: Record<string, any>): Promise<T> {
    const url = new URL(endpoint, this.baseURL)
    if (params) {
      Object.entries(params).forEach(([key, value]) => {
        url.searchParams.append(key, String(value))
      })
    }
    return this.request<T>(url.toString())
  }

  post<T>(endpoint: string, data?: any): Promise<T> {
    return this.request<T>(endpoint, {
      method: 'POST',
      body: JSON.stringify(data),
    })
  }
}

export const httpClient = new HttpClient(API_BASE_URL)
```

### 2. 类型安全的 API 调用

```typescript
// src/api/types.ts
export interface ApiResponse<T = any> {
  code: number
  message: string
  data: T
}

export interface LoginRequest {
  username: string
  password: string
}

export interface LoginResponse {
  token: string
  user: {
    id: string
    username: string
    email: string
  }
}

// src/api/authApi.ts
import { ApiEndpoints } from '@/config'
import { httpClient } from '@/utils/httpClient'
import type { ApiResponse, LoginRequest, LoginResponse } from './types'

export async function login(credentials: LoginRequest): Promise<ApiResponse<LoginResponse>> {
  return httpClient.post<ApiResponse<LoginResponse>>(
    ApiEndpoints.AUTH.LOGIN,
    credentials
  )
}
```

---

## 常见问题

### Q: 如何在不同环境使用不同的 API 地址？

A: 使用环境变量覆盖默认配置：

```bash
# .env.development
VITE_API_BASE_URL=http://localhost:8080

# .env.production
VITE_API_BASE_URL=https://api.qutab.cn
```

### Q: 如何添加认证 Token？

A: 在请求拦截器中添加：

```typescript
const token = localStorage.getItem('token')
if (token) {
  options.headers = {
    ...options.headers,
    'Authorization': `Bearer ${token}`
  }
}
```

### Q: 如何处理 API 错误？

A: 创建统一的错误处理函数：

```typescript
function handleApiError(error: any) {
  if (error.code === 401) {
    // Token 过期，跳转登录
    router.push('/login')
  } else if (error.code === 403) {
    // 无权限
    ElMessage.error('您没有权限执行此操作')
  } else {
    ElMessage.error(error.message || '请求失败')
  }
}
```

---

**文档维护者:** 触宝开发团队  
**最后更新:** 2026-02-07
