// Window Helper - 窗口识别与过滤增强
#[cfg(windows)]
use windows::Win32::Foundation::{HWND, LPARAM, BOOL, RECT};
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowThreadProcessId, GetWindowTextW, SetWindowTextW,
    IsWindowVisible, GetClassNameW, GetWindowLongW, GetWindowRect,
    SetWindowPos, ShowWindow, 
    GWL_STYLE, WS_CAPTION, WS_VISIBLE,
    HWND_TOP, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW,
    SW_HIDE, SW_SHOW, SW_MINIMIZE, SW_RESTORE,
};
use tracing::{info, warn, debug};
use serde::{Serialize, Deserialize};

/// 窗口详细信息（可序列化，用于前端展示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowDetail {
    pub hwnd_ptr: isize,
    pub title: String,
    pub class_name: String,
    pub is_visible: bool,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub pid: u32,
}

#[cfg(windows)]
pub struct WindowInfo {
    pub hwnd: HWND,
    pub title: String,
    pub class_name: String,
    pub is_visible: bool,
    pub style: i32,
    pub rect: RECT,
    pub pid: u32,
}

#[cfg(windows)]
impl WindowInfo {
    /// 获取窗口信息
    pub unsafe fn from_hwnd(hwnd: HWND) -> Option<Self> {
        // 获取标题
        let mut title_buf = vec![0u16; 256];
        let title_len = GetWindowTextW(hwnd, &mut title_buf);
        let title = String::from_utf16_lossy(&title_buf[..title_len as usize]);

        // 获取类名
        let mut class_buf = vec![0u16; 256];
        let class_len = GetClassNameW(hwnd, &mut class_buf);
        let class_name = String::from_utf16_lossy(&class_buf[..class_len as usize]);

        // 获取可见性
        let is_visible = IsWindowVisible(hwnd).as_bool();

        // 获取样式
        let style = GetWindowLongW(hwnd, GWL_STYLE);
        
        // 获取窗口位置和尺寸
        let mut rect = RECT::default();
        let _ = GetWindowRect(hwnd, &mut rect);
        
        // 获取进程 ID
        let mut pid: u32 = 0;
        let _ = GetWindowThreadProcessId(hwnd, Some(&mut pid));

        Some(WindowInfo {
            hwnd,
            title,
            class_name,
            is_visible,
            style,
            rect,
            pid,
        })
    }
    
    /// 获取窗口宽度
    pub fn width(&self) -> i32 {
        self.rect.right - self.rect.left
    }
    
    /// 获取窗口高度
    pub fn height(&self) -> i32 {
        self.rect.bottom - self.rect.top
    }
    
    /// 转换为可序列化的 WindowDetail
    pub fn to_detail(&self) -> WindowDetail {
        WindowDetail {
            hwnd_ptr: self.hwnd.0 as isize,
            title: self.title.clone(),
            class_name: self.class_name.clone(),
            is_visible: self.is_visible,
            x: self.rect.left,
            y: self.rect.top,
            width: self.width(),
            height: self.height(),
            pid: self.pid,
        }
    }

    /// 判断是否为主浏览器窗口（增强版）
    pub fn is_main_browser_window(&self) -> bool {
        // 必须可见
        if !self.is_visible {
            return false;
        }

        // 必须有标题（排除空白窗口）
        if self.title.is_empty() {
            return false;
        }

        // 必须有标题栏样式
        if (self.style & WS_CAPTION.0 as i32) == 0 {
            return false;
        }
        
        // ✅ 新增：窗口尺寸过滤（排除小窗口，如弹窗、通知等）
        let min_width = 400;
        let min_height = 300;
        if self.width() < min_width || self.height() < min_height {
            debug!(
                title = %self.title, 
                width = self.width(), 
                height = self.height(),
                "排除小窗口"
            );
            return false;
        }
        
        // ✅ 新增：严格的 Chrome 类名匹配
        // Chrome 主窗口类名为 "Chrome_WidgetWin_1"
        // "Chrome_WidgetWin_0" 通常是隐藏窗口或辅助窗口
        if !self.class_name.starts_with("Chrome_WidgetWin_") {
            return false;
        }

        // 排除 DevTools 窗口（通过标题判断）
        let title_lower = self.title.to_lowercase();
        if title_lower.contains("devtools") 
            || title_lower.contains("developer tools")
            || title_lower.contains("开发者工具") {
            return false;
        }
        
        // 排除任务管理器窗口
        if title_lower.contains("task manager") 
            || title_lower.contains("任务管理器") {
            return false;
        }
        
        // 排除设置/历史/下载等内部页面（可选，根据需求）
        // 注释掉以保留这些窗口的控制能力
        // if title_lower.starts_with("chrome://") {
        //     return false;
        // }

        // 排除扩展窗口（通过标题判断）
        if title_lower.contains("extension:") 
            || title_lower.starts_with("chrome-extension://") {
            return false;
        }

        // 排除 Chrome 的一些特殊窗口（空标题 + WidgetWin_0）
        if self.class_name == "Chrome_WidgetWin_0" && self.title.is_empty() {
            return false;
        }

        true
    }
}

#[cfg(windows)]
struct EnumWindowsContext {
    pids: Vec<u32>,
    windows: Vec<WindowInfo>,
}

#[cfg(windows)]
unsafe extern "system" fn enum_windows_callback_enhanced(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let ctx = &mut *(lparam.0 as *mut EnumWindowsContext);
    let mut pid: u32 = 0;
    let _ = GetWindowThreadProcessId(hwnd, Some(&mut pid));
    
    if ctx.pids.contains(&pid) {
        if let Some(info) = WindowInfo::from_hwnd(hwnd) {
            ctx.windows.push(info);
        }
    }
    
    BOOL(1)
}

/// 收集指定进程的主窗口句柄
#[cfg(windows)]
pub fn collect_main_windows_for_pids(pids: &[u32]) -> Vec<HWND> {
    let mut ctx = EnumWindowsContext {
        pids: pids.to_vec(),
        windows: Vec::new(),
    };
    
    unsafe {
        let _ = EnumWindows(
            Some(enum_windows_callback_enhanced),
            LPARAM((&mut ctx as *mut EnumWindowsContext) as isize),
        );
    }
    
    // 过滤出主窗口
    ctx.windows
        .into_iter()
        .filter(|w| w.is_main_browser_window())
        .map(|w| w.hwnd)
        .collect()
}

/// 收集指定进程的所有窗口详细信息
#[cfg(windows)]
pub fn collect_window_details_for_pids(pids: &[u32]) -> Vec<WindowDetail> {
    let mut ctx = EnumWindowsContext {
        pids: pids.to_vec(),
        windows: Vec::new(),
    };
    
    unsafe {
        let _ = EnumWindows(
            Some(enum_windows_callback_enhanced),
            LPARAM((&mut ctx as *mut EnumWindowsContext) as isize),
        );
    }
    
    // 过滤并转换为详细信息
    ctx.windows
        .into_iter()
        .filter(|w| w.is_main_browser_window())
        .map(|w| w.to_detail())
        .collect()
}

/// 带重试的窗口收集（异步）
#[cfg(windows)]
pub async fn collect_main_windows_with_retry(
    pids: &[u32],
    max_retries: u32,
    retry_delay_ms: u64,
) -> Vec<HWND> {
    use tokio::time::{sleep, Duration};
    
    for attempt in 0..max_retries {
        let hwnds = collect_main_windows_for_pids(pids);
        
        if !hwnds.is_empty() {
            if attempt > 0 {
                debug!(attempt = attempt, count = hwnds.len(), "窗口识别成功");
            }
            return hwnds;
        }
        
        if attempt < max_retries - 1 {
            debug!(attempt = attempt, delay_ms = retry_delay_ms, "窗口未就绪，等待后重试");
            sleep(Duration::from_millis(retry_delay_ms)).await;
        }
    }
    
    warn!(max_retries = max_retries, "经过多次重试后仍未找到窗口");
    Vec::new()
}

/// 带重试的窗口详情收集（异步）
#[cfg(windows)]
pub async fn collect_window_details_with_retry(
    pids: &[u32],
    max_retries: u32,
    retry_delay_ms: u64,
) -> Vec<WindowDetail> {
    use tokio::time::{sleep, Duration};
    
    for attempt in 0..max_retries {
        let details = collect_window_details_for_pids(pids);
        
        if !details.is_empty() {
            if attempt > 0 {
                debug!(attempt = attempt, count = details.len(), "窗口详情获取成功");
            }
            return details;
        }
        
        if attempt < max_retries - 1 {
            sleep(Duration::from_millis(retry_delay_ms)).await;
        }
    }
    
    Vec::new()
}

// ==================== 窗口操作功能 ====================

/// 重命名窗口标题
#[cfg(windows)]
pub fn rename_window(hwnd_ptr: isize, new_title: &str) -> Result<(), String> {
    let hwnd = HWND(hwnd_ptr as *mut std::ffi::c_void);
    
    // 转换为宽字符
    let wide_title: Vec<u16> = new_title.encode_utf16().chain(std::iter::once(0)).collect();
    
    unsafe {
        SetWindowTextW(hwnd, windows::core::PCWSTR(wide_title.as_ptr()))
            .map_err(|e| format!("设置窗口标题失败: hwnd={}, error={}", hwnd_ptr, e))?;
        info!(hwnd = hwnd_ptr, title = new_title, "窗口标题已更新");
        Ok(())
    }
}

/// 置顶窗口（激活并显示在最前）
#[cfg(windows)]
pub fn bring_window_to_front(hwnd_ptr: isize) -> Result<(), String> {
    let hwnd = HWND(hwnd_ptr as *mut std::ffi::c_void);
    
    unsafe {
        // 恢复窗口（如果最小化）
        let _ = ShowWindow(hwnd, SW_RESTORE);
        
        // 置顶
        SetWindowPos(
            hwnd,
            HWND_TOP,
            0, 0, 0, 0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
        ).map_err(|e| format!("置顶窗口失败: hwnd={}, error={}", hwnd_ptr, e))?;
        
        debug!(hwnd = hwnd_ptr, "窗口已置顶");
        Ok(())
    }
}

/// 最小化窗口
#[cfg(windows)]
pub fn minimize_window(hwnd_ptr: isize) -> Result<(), String> {
    let hwnd = HWND(hwnd_ptr as *mut std::ffi::c_void);
    
    unsafe {
        // ShowWindow 返回的是之前的可见状态，不是操作结果
        let _ = ShowWindow(hwnd, SW_MINIMIZE);
        debug!(hwnd = hwnd_ptr, "窗口已最小化");
        Ok(())
    }
}

/// 恢复窗口（从最小化恢复）
#[cfg(windows)]
pub fn restore_window(hwnd_ptr: isize) -> Result<(), String> {
    let hwnd = HWND(hwnd_ptr as *mut std::ffi::c_void);
    
    unsafe {
        let _ = ShowWindow(hwnd, SW_RESTORE);
        Ok(())
    }
}

/// 隐藏窗口
#[cfg(windows)]
pub fn hide_window(hwnd_ptr: isize) -> Result<(), String> {
    let hwnd = HWND(hwnd_ptr as *mut std::ffi::c_void);
    
    unsafe {
        let _ = ShowWindow(hwnd, SW_HIDE);
        debug!(hwnd = hwnd_ptr, "窗口已隐藏");
        Ok(())
    }
}

/// 显示窗口
#[cfg(windows)]
pub fn show_window(hwnd_ptr: isize) -> Result<(), String> {
    let hwnd = HWND(hwnd_ptr as *mut std::ffi::c_void);
    
    unsafe {
        let _ = ShowWindow(hwnd, SW_SHOW);
        debug!(hwnd = hwnd_ptr, "窗口已显示");
        Ok(())
    }
}

/// 移动窗口到指定位置
#[cfg(windows)]
pub fn move_window(hwnd_ptr: isize, x: i32, y: i32) -> Result<(), String> {
    use windows::Win32::UI::WindowsAndMessaging::SWP_NOACTIVATE;
    
    let hwnd = HWND(hwnd_ptr as *mut std::ffi::c_void);
    
    unsafe {
        SetWindowPos(
            hwnd,
            HWND_TOP,
            x, y, 0, 0,
            SWP_NOSIZE | SWP_NOACTIVATE,
        ).map_err(|e| format!("移动窗口失败: hwnd={}, error={}", hwnd_ptr, e))?;
        
        debug!(hwnd = hwnd_ptr, x = x, y = y, "窗口已移动");
        Ok(())
    }
}

/// 调整窗口大小
#[cfg(windows)]
pub fn resize_window(hwnd_ptr: isize, width: i32, height: i32) -> Result<(), String> {
    use windows::Win32::UI::WindowsAndMessaging::SWP_NOACTIVATE;
    
    let hwnd = HWND(hwnd_ptr as *mut std::ffi::c_void);
    
    unsafe {
        SetWindowPos(
            hwnd,
            HWND_TOP,
            0, 0, width, height,
            SWP_NOMOVE | SWP_NOACTIVATE,
        ).map_err(|e| format!("调整窗口大小失败: hwnd={}, error={}", hwnd_ptr, e))?;
        
        debug!(hwnd = hwnd_ptr, width = width, height = height, "窗口大小已调整");
        Ok(())
    }
}

/// 移动并调整窗口
#[cfg(windows)]
pub fn set_window_bounds(hwnd_ptr: isize, x: i32, y: i32, width: i32, height: i32) -> Result<(), String> {
    use windows::Win32::UI::WindowsAndMessaging::SWP_NOACTIVATE;
    
    let hwnd = HWND(hwnd_ptr as *mut std::ffi::c_void);
    
    unsafe {
        SetWindowPos(
            hwnd,
            HWND_TOP,
            x, y, width, height,
            SWP_NOACTIVATE,
        ).map_err(|e| format!("设置窗口位置和大小失败: hwnd={}, error={}", hwnd_ptr, e))?;
        
        debug!(hwnd = hwnd_ptr, x = x, y = y, width = width, height = height, "窗口位置和大小已设置");
        Ok(())
    }
}

// ==================== 非 Windows 平台的占位实现 ====================

#[cfg(not(windows))]
pub fn collect_main_windows_for_pids(_pids: &[u32]) -> Vec<()> {
    Vec::new()
}

#[cfg(not(windows))]
pub fn collect_window_details_for_pids(_pids: &[u32]) -> Vec<WindowDetail> {
    Vec::new()
}

#[cfg(not(windows))]
pub async fn collect_main_windows_with_retry(_pids: &[u32], _max_retries: u32, _retry_delay_ms: u64) -> Vec<()> {
    Vec::new()
}

#[cfg(not(windows))]
pub async fn collect_window_details_with_retry(_pids: &[u32], _max_retries: u32, _retry_delay_ms: u64) -> Vec<WindowDetail> {
    Vec::new()
}

#[cfg(not(windows))]
pub fn rename_window(_hwnd_ptr: isize, _new_title: &str) -> Result<(), String> {
    Err("窗口操作仅支持 Windows 平台".to_string())
}

#[cfg(not(windows))]
pub fn bring_window_to_front(_hwnd_ptr: isize) -> Result<(), String> {
    Err("窗口操作仅支持 Windows 平台".to_string())
}

#[cfg(not(windows))]
pub fn minimize_window(_hwnd_ptr: isize) -> Result<(), String> {
    Err("窗口操作仅支持 Windows 平台".to_string())
}

#[cfg(not(windows))]
pub fn restore_window(_hwnd_ptr: isize) -> Result<(), String> {
    Err("窗口操作仅支持 Windows 平台".to_string())
}

#[cfg(not(windows))]
pub fn hide_window(_hwnd_ptr: isize) -> Result<(), String> {
    Err("窗口操作仅支持 Windows 平台".to_string())
}

#[cfg(not(windows))]
pub fn show_window(_hwnd_ptr: isize) -> Result<(), String> {
    Err("窗口操作仅支持 Windows 平台".to_string())
}

#[cfg(not(windows))]
pub fn move_window(_hwnd_ptr: isize, _x: i32, _y: i32) -> Result<(), String> {
    Err("窗口操作仅支持 Windows 平台".to_string())
}

#[cfg(not(windows))]
pub fn resize_window(_hwnd_ptr: isize, _width: i32, _height: i32) -> Result<(), String> {
    Err("窗口操作仅支持 Windows 平台".to_string())
}

#[cfg(not(windows))]
pub fn set_window_bounds(_hwnd_ptr: isize, _x: i32, _y: i32, _width: i32, _height: i32) -> Result<(), String> {
    Err("窗口操作仅支持 Windows 平台".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(windows)]
    fn test_window_info_is_main_browser_window() {
        // 创建模拟的主窗口（符合所有条件）
        let main_window = WindowInfo {
            hwnd: HWND(std::ptr::null_mut()),
            title: "Google Chrome".to_string(),
            class_name: "Chrome_WidgetWin_1".to_string(),
            is_visible: true,
            style: (WS_VISIBLE.0 | WS_CAPTION.0) as i32,
            rect: RECT { left: 0, top: 0, right: 1920, bottom: 1080 },
            pid: 1234,
        };
        
        assert!(main_window.is_main_browser_window());
        
        // DevTools 窗口应该被排除
        let devtools_window = WindowInfo {
            hwnd: HWND(std::ptr::null_mut()),
            title: "DevTools - Google Chrome".to_string(),
            class_name: "Chrome_WidgetWin_1".to_string(),
            is_visible: true,
            style: (WS_VISIBLE.0 | WS_CAPTION.0) as i32,
            rect: RECT { left: 0, top: 0, right: 800, bottom: 600 },
            pid: 1234,
        };
        
        assert!(!devtools_window.is_main_browser_window());
        
        // 小窗口应该被排除
        let small_window = WindowInfo {
            hwnd: HWND(std::ptr::null_mut()),
            title: "Popup".to_string(),
            class_name: "Chrome_WidgetWin_1".to_string(),
            is_visible: true,
            style: (WS_VISIBLE.0 | WS_CAPTION.0) as i32,
            rect: RECT { left: 0, top: 0, right: 300, bottom: 200 },
            pid: 1234,
        };
        
        assert!(!small_window.is_main_browser_window());
        
        // 非 Chrome 类名应该被排除
        let non_chrome_window = WindowInfo {
            hwnd: HWND(std::ptr::null_mut()),
            title: "Some Window".to_string(),
            class_name: "SomeOtherClass".to_string(),
            is_visible: true,
            style: (WS_VISIBLE.0 | WS_CAPTION.0) as i32,
            rect: RECT { left: 0, top: 0, right: 1920, bottom: 1080 },
            pid: 1234,
        };
        
        assert!(!non_chrome_window.is_main_browser_window());
    }
    
    #[test]
    fn test_window_detail_serialization() {
        let detail = WindowDetail {
            hwnd_ptr: 12345,
            title: "Test Window".to_string(),
            class_name: "Chrome_WidgetWin_1".to_string(),
            is_visible: true,
            x: 100,
            y: 100,
            width: 1200,
            height: 800,
            pid: 5678,
        };
        
        // 测试序列化
        let json = serde_json::to_string(&detail).unwrap();
        assert!(json.contains("Test Window"));
        assert!(json.contains("12345"));
        
        // 测试反序列化
        let parsed: WindowDetail = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.title, "Test Window");
        assert_eq!(parsed.hwnd_ptr, 12345);
    }
}
