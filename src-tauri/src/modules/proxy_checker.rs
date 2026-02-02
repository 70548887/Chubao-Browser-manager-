//! Proxy Health Checker Module
//! 
//! 提供完整的代理健康检查功能：
//! - HTTP/HTTPS 代理检测
//! - SOCKS5 代理检测
//! - 出口 IP 和地理位置获取
//! - 批量检测支持

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tracing::{info, warn, debug, error};

/// IP-API.com 响应结构
#[derive(Debug, Deserialize)]
struct IpApiResponse {
    status: String,
    #[serde(default)]
    query: String,
    #[serde(default)]
    country: String,
    #[serde(default)]
    #[serde(rename = "countryCode")]
    country_code: String,
    #[serde(default)]
    city: String,
    #[serde(default)]
    isp: String,
    #[serde(default)]
    message: String,
}

/// 代理检测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyCheckResult {
    /// 代理 ID
    pub proxy_id: String,
    /// 检测是否成功
    pub success: bool,
    /// 延迟（毫秒）
    pub latency: Option<u64>,
    /// 出口 IP
    pub ip: Option<String>,
    /// 国家
    pub country: Option<String>,
    /// 国家代码
    pub country_code: Option<String>,
    /// 城市
    pub city: Option<String>,
    /// ISP 运营商
    pub isp: Option<String>,
    /// 格式化的位置信息
    pub location: Option<String>,
    /// 错误信息
    pub error: Option<String>,
    /// 检测时间戳
    pub checked_at: String,
}

impl ProxyCheckResult {
    /// 创建成功结果
    pub fn success(
        proxy_id: String,
        latency: u64,
        ip: String,
        country: String,
        country_code: String,
        city: String,
        isp: String,
    ) -> Self {
        let location = if city.is_empty() {
            country.clone()
        } else {
            format!("{}, {}", city, country)
        };
        
        Self {
            proxy_id,
            success: true,
            latency: Some(latency),
            ip: Some(ip),
            country: Some(country),
            country_code: Some(country_code),
            city: Some(city),
            isp: Some(isp),
            location: Some(location),
            error: None,
            checked_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// 创建失败结果
    pub fn failure(proxy_id: String, error: String) -> Self {
        Self {
            proxy_id,
            success: false,
            latency: None,
            ip: None,
            country: None,
            country_code: None,
            city: None,
            isp: None,
            location: None,
            error: Some(error),
            checked_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// 代理检测器
pub struct ProxyChecker {
    /// 检测超时时间（秒）
    timeout_secs: u64,
    /// IP 查询服务 URL
    ip_api_url: String,
}

impl Default for ProxyChecker {
    fn default() -> Self {
        Self {
            timeout_secs: 10,
            ip_api_url: "http://ip-api.com/json".to_string(),
        }
    }
}

impl ProxyChecker {
    /// 创建新的检测器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置超时时间
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// 检测 HTTP/HTTPS 代理
    pub async fn check_http_proxy(
        &self,
        proxy_id: &str,
        host: &str,
        port: &str,
        username: Option<&str>,
        password: Option<&str>,
    ) -> ProxyCheckResult {
        debug!(proxy_id = proxy_id, host = host, port = port, "开始检测 HTTP 代理");
        
        let proxy_url = self.build_proxy_url("http", host, port, username, password);
        self.do_check(proxy_id, &proxy_url).await
    }

    /// 检测 HTTPS 代理
    pub async fn check_https_proxy(
        &self,
        proxy_id: &str,
        host: &str,
        port: &str,
        username: Option<&str>,
        password: Option<&str>,
    ) -> ProxyCheckResult {
        debug!(proxy_id = proxy_id, host = host, port = port, "开始检测 HTTPS 代理");
        
        let proxy_url = self.build_proxy_url("https", host, port, username, password);
        self.do_check(proxy_id, &proxy_url).await
    }

    /// 检测 SOCKS5 代理
    pub async fn check_socks5_proxy(
        &self,
        proxy_id: &str,
        host: &str,
        port: &str,
        username: Option<&str>,
        password: Option<&str>,
    ) -> ProxyCheckResult {
        debug!(proxy_id = proxy_id, host = host, port = port, "开始检测 SOCKS5 代理");
        
        let proxy_url = self.build_proxy_url("socks5", host, port, username, password);
        self.do_check(proxy_id, &proxy_url).await
    }

    /// 根据代理类型自动选择检测方法
    pub async fn check_proxy(
        &self,
        proxy_id: &str,
        proxy_type: &str,
        host: &str,
        port: &str,
        username: Option<&str>,
        password: Option<&str>,
    ) -> ProxyCheckResult {
        match proxy_type.to_lowercase().as_str() {
            "http" => self.check_http_proxy(proxy_id, host, port, username, password).await,
            "https" => self.check_https_proxy(proxy_id, host, port, username, password).await,
            "socks5" => self.check_socks5_proxy(proxy_id, host, port, username, password).await,
            "direct" => {
                // 直连模式，检测本机出口 IP
                self.check_direct(proxy_id).await
            }
            _ => ProxyCheckResult::failure(
                proxy_id.to_string(),
                format!("不支持的代理类型: {}", proxy_type),
            ),
        }
    }

    /// 检测直连模式（无代理）
    async fn check_direct(&self, proxy_id: &str) -> ProxyCheckResult {
        debug!(proxy_id = proxy_id, "检测直连模式");
        
        let start = Instant::now();
        
        let client = match reqwest::Client::builder()
            .timeout(Duration::from_secs(self.timeout_secs))
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                return ProxyCheckResult::failure(
                    proxy_id.to_string(),
                    format!("创建 HTTP 客户端失败: {}", e),
                );
            }
        };

        match client.get(&self.ip_api_url).send().await {
            Ok(response) => {
                let latency = start.elapsed().as_millis() as u64;
                self.parse_ip_api_response(proxy_id, response, latency).await
            }
            Err(e) => ProxyCheckResult::failure(
                proxy_id.to_string(),
                format!("请求失败: {}", e),
            ),
        }
    }

    /// 构建代理 URL
    fn build_proxy_url(
        &self,
        scheme: &str,
        host: &str,
        port: &str,
        username: Option<&str>,
        password: Option<&str>,
    ) -> String {
        match (username, password) {
            (Some(user), Some(pass)) if !user.is_empty() && !pass.is_empty() => {
                format!("{}://{}:{}@{}:{}", scheme, user, pass, host, port)
            }
            (Some(user), _) if !user.is_empty() => {
                format!("{}://{}@{}:{}", scheme, user, host, port)
            }
            _ => {
                format!("{}://{}:{}", scheme, host, port)
            }
        }
    }

    /// 执行实际的代理检测
    async fn do_check(&self, proxy_id: &str, proxy_url: &str) -> ProxyCheckResult {
        let start = Instant::now();
        
        // 构建代理
        let proxy = match reqwest::Proxy::all(proxy_url) {
            Ok(p) => p,
            Err(e) => {
                warn!(proxy_id = proxy_id, error = %e, "代理 URL 解析失败");
                return ProxyCheckResult::failure(
                    proxy_id.to_string(),
                    format!("代理配置无效: {}", e),
                );
            }
        };

        // 构建 HTTP 客户端
        let client = match reqwest::Client::builder()
            .proxy(proxy)
            .timeout(Duration::from_secs(self.timeout_secs))
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                warn!(proxy_id = proxy_id, error = %e, "创建 HTTP 客户端失败");
                return ProxyCheckResult::failure(
                    proxy_id.to_string(),
                    format!("创建 HTTP 客户端失败: {}", e),
                );
            }
        };

        // 发送请求获取出口 IP 信息
        match client.get(&self.ip_api_url).send().await {
            Ok(response) => {
                let latency = start.elapsed().as_millis() as u64;
                info!(proxy_id = proxy_id, latency = latency, "代理检测成功");
                self.parse_ip_api_response(proxy_id, response, latency).await
            }
            Err(e) => {
                let error_msg = if e.is_timeout() {
                    format!("连接超时（{}秒）", self.timeout_secs)
                } else if e.is_connect() {
                    "连接失败，请检查代理服务器是否可用".to_string()
                } else {
                    format!("请求失败: {}", e)
                };
                
                warn!(proxy_id = proxy_id, error = %e, "代理检测失败");
                ProxyCheckResult::failure(proxy_id.to_string(), error_msg)
            }
        }
    }

    /// 解析 IP-API 响应
    async fn parse_ip_api_response(
        &self,
        proxy_id: &str,
        response: reqwest::Response,
        latency: u64,
    ) -> ProxyCheckResult {
        match response.json::<IpApiResponse>().await {
            Ok(ip_info) => {
                if ip_info.status == "success" {
                    ProxyCheckResult::success(
                        proxy_id.to_string(),
                        latency,
                        ip_info.query,
                        ip_info.country,
                        ip_info.country_code,
                        ip_info.city,
                        ip_info.isp,
                    )
                } else {
                    ProxyCheckResult::failure(
                        proxy_id.to_string(),
                        format!("IP 查询失败: {}", ip_info.message),
                    )
                }
            }
            Err(e) => {
                warn!(proxy_id = proxy_id, error = %e, "解析 IP 信息失败");
                // 即使解析失败，连接是成功的，返回部分信息
                ProxyCheckResult {
                    proxy_id: proxy_id.to_string(),
                    success: true,
                    latency: Some(latency),
                    ip: None,
                    country: None,
                    country_code: None,
                    city: None,
                    isp: None,
                    location: Some("连接成功，但无法获取 IP 信息".to_string()),
                    error: None,
                    checked_at: chrono::Utc::now().to_rfc3339(),
                }
            }
        }
    }

    /// 批量检测代理
    pub async fn batch_check<'a>(
        &self,
        proxies: Vec<ProxyInfo<'a>>,
        concurrency: usize,
    ) -> Vec<ProxyCheckResult> {
        use futures::stream::{self, StreamExt};
        
        let results = stream::iter(proxies)
            .map(|proxy| async move {
                self.check_proxy(
                    proxy.id,
                    proxy.proxy_type,
                    proxy.host,
                    proxy.port,
                    proxy.username,
                    proxy.password,
                ).await
            })
            .buffer_unordered(concurrency)
            .collect::<Vec<_>>()
            .await;
        
        results
    }
}

/// 代理信息（用于批量检测）
#[derive(Debug)]
pub struct ProxyInfo<'a> {
    pub id: &'a str,
    pub proxy_type: &'a str,
    pub host: &'a str,
    pub port: &'a str,
    pub username: Option<&'a str>,
    pub password: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_direct() {
        let checker = ProxyChecker::new().with_timeout(5);
        let result = checker.check_direct("test-direct").await;
        
        // 注意：这个测试需要网络连接
        println!("Direct check result: {:?}", result);
    }

    #[test]
    fn test_build_proxy_url() {
        let checker = ProxyChecker::new();
        
        assert_eq!(
            checker.build_proxy_url("http", "127.0.0.1", "8080", None, None),
            "http://127.0.0.1:8080"
        );
        
        assert_eq!(
            checker.build_proxy_url("socks5", "proxy.example.com", "1080", Some("user"), Some("pass")),
            "socks5://user:pass@proxy.example.com:1080"
        );
    }
}
