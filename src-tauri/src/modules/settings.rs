// Settings 模块 - 设置管理与校验
use std::path::Path;

/// 校验 kernel_path 设置
pub fn validate_kernel_path(path: &str) -> Result<(), String> {
    let path_obj = Path::new(path);
    
    // 检查路径是否存在
    if !path_obj.exists() {
        return Err(format!("浏览器内核路径不存在: {}", path));
    }
    
    // 检查是否为文件
    if !path_obj.is_file() {
        return Err(format!("浏览器内核路径不是文件: {}", path));
    }
    
    // Windows 平台检查 .exe 扩展名
    #[cfg(target_os = "windows")]
    {
        if path_obj.extension().and_then(|s| s.to_str()) != Some("exe") {
            return Err("浏览器内核文件必须是 .exe 可执行文件".to_string());
        }
    }
    
    // Unix 系统检查执行权限
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = path_obj.metadata()
            .map_err(|e| format!("无法读取文件权限: {}", e))?;
        let permissions = metadata.permissions();
        if permissions.mode() & 0o111 == 0 {
            return Err("浏览器内核文件没有执行权限".to_string());
        }
    }
    
    Ok(())
}

/// 校验 user_data_dir 设置
pub fn validate_user_data_dir(path: &str) -> Result<(), String> {
    let path_obj = Path::new(path);
    
    // 如果目录不存在，尝试创建
    if !path_obj.exists() {
        std::fs::create_dir_all(path_obj)
            .map_err(|e| format!("无法创建用户数据目录: {}", e))?;
    }
    
    // 检查是否为目录
    if !path_obj.is_dir() {
        return Err("用户数据路径必须是目录".to_string());
    }
    
    // 测试写入权限
    let test_file = path_obj.join(".write_test");
    std::fs::write(&test_file, "test")
        .map_err(|e| format!("用户数据目录不可写: {}", e))?;
    std::fs::remove_file(&test_file)
        .map_err(|e| format!("清理测试文件失败: {}", e))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_validate_kernel_path_not_exists() {
        let result = validate_kernel_path("/nonexistent/path/chrome.exe");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("不存在"));
    }

    #[test]
    fn test_validate_kernel_path_is_directory() {
        let temp_dir = TempDir::new().unwrap();
        let result = validate_kernel_path(temp_dir.path().to_str().unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("不是文件"));
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_validate_kernel_path_wrong_extension() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("chrome.txt");
        fs::write(&file_path, "test").unwrap();
        
        let result = validate_kernel_path(file_path.to_str().unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(".exe"));
    }

    #[test]
    fn test_validate_user_data_dir_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let new_dir = temp_dir.path().join("new_data_dir");
        
        let result = validate_user_data_dir(new_dir.to_str().unwrap());
        assert!(result.is_ok());
        assert!(new_dir.exists());
    }

    #[test]
    fn test_validate_user_data_dir_writable() {
        let temp_dir = TempDir::new().unwrap();
        let result = validate_user_data_dir(temp_dir.path().to_str().unwrap());
        assert!(result.is_ok());
    }
}
