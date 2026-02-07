// Proxy Bridge - 代理桥接模块
// 解决 Chromium 原生不支持带密码 SOCKS5 代理的问题
// 架构：Chrome → 本地 HTTP 代理 (127.0.0.1:port) → 上游 SOCKS5 代理 (含认证)

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use tracing::{info, warn, error, debug};
use serde::{Serialize, Deserialize};

/// 代理桥接配置
#[derive(Debug, Clone)]
pub struct ProxyBridgeConfig {
    /// 上游代理主机
    pub upstream_host: String,
    /// 上游代理端口
    pub upstream_port: u16,
    /// 上游代理协议 (socks5, http, https)
    pub upstream_type: String,
    /// 上游代理用户名（可选）
    pub username: Option<String>,
    /// 上游代理密码（可选）
    pub password: Option<String>,
    /// 是否启用 UDP 支持（用于 WebRTC）
    pub enable_udp: bool,
}

/// 代理桥接统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeStats {
    /// 总连接数
    pub total_connections: u64,
    /// 活跃连接数
    pub active_connections: u64,
    /// 上行字节数
    pub bytes_sent: u64,
    /// 下行字节数
    pub bytes_received: u64,
    /// 失败连接数
    pub failed_connections: u64,
    /// 启动时间
    pub started_at: i64,
    /// UDP 数据包发送数
    pub udp_packets_sent: u64,
    /// UDP 数据包接收数
    pub udp_packets_received: u64,
}

/// 代理桥接实例
pub struct ProxyBridge {
    /// Profile ID（对应）
    pub profile_id: String,
    /// 本地 TCP 监听端口
    pub local_port: u16,
    /// 本地 UDP 监听端口（用于 WebRTC）
    pub local_udp_port: u16,
    /// 上游配置
    config: ProxyBridgeConfig,
    /// 运行状态
    running: Arc<AtomicBool>,
    /// 统计信息
    stats: Arc<BridgeStatsInner>,
}

/// 内部统计结构（支持原子更新）
struct BridgeStatsInner {
    total_connections: AtomicU64,
    active_connections: AtomicU64,
    bytes_sent: AtomicU64,
    bytes_received: AtomicU64,
    failed_connections: AtomicU64,
    started_at: i64,
    udp_packets_sent: AtomicU64,
    udp_packets_received: AtomicU64,
}

impl BridgeStatsInner {
    fn new() -> Self {
        Self {
            total_connections: AtomicU64::new(0),
            active_connections: AtomicU64::new(0),
            bytes_sent: AtomicU64::new(0),
            bytes_received: AtomicU64::new(0),
            failed_connections: AtomicU64::new(0),
            started_at: chrono::Utc::now().timestamp_millis(),
            udp_packets_sent: AtomicU64::new(0),
            udp_packets_received: AtomicU64::new(0),
        }
    }
    
    fn to_stats(&self) -> BridgeStats {
        BridgeStats {
            total_connections: self.total_connections.load(Ordering::Relaxed),
            active_connections: self.active_connections.load(Ordering::Relaxed),
            bytes_sent: self.bytes_sent.load(Ordering::Relaxed),
            bytes_received: self.bytes_received.load(Ordering::Relaxed),
            failed_connections: self.failed_connections.load(Ordering::Relaxed),
            started_at: self.started_at,
            udp_packets_sent: self.udp_packets_sent.load(Ordering::Relaxed),
            udp_packets_received: self.udp_packets_received.load(Ordering::Relaxed),
        }
    }
}

impl ProxyBridge {
    /// 创建新的代理桥接实例
    pub fn new(profile_id: String, config: ProxyBridgeConfig) -> Self {
        let local_port = Self::find_free_port(50000);
        let local_udp_port = if config.enable_udp {
            Self::find_free_port(local_port + 1) // UDP 端口在 TCP 端口后面
        } else {
            0
        };
        
        Self {
            profile_id,
            local_port,
            local_udp_port,
            config,
            running: Arc::new(AtomicBool::new(false)),
            stats: Arc::new(BridgeStatsInner::new()),
        }
    }
    
    /// 查找可用端口（从指定端口开始）
    fn find_free_port(start: u16) -> u16 {
        for port in start..60000 {
            if std::net::TcpListener::bind(("127.0.0.1", port)).is_ok() {
                return port;
            }
        }
        // 如果所有端口都不可用，返回 0 让系统分配
        0
    }
    
    /// 获取本地 TCP 代理地址（供浏览器 HTTP 流量使用）
    pub fn local_addr(&self) -> String {
        format!("http://127.0.0.1:{}", self.local_port)
    }
    
    /// 获取本地 UDP 代理地址（供 WebRTC 使用）
    pub fn local_udp_addr(&self) -> Option<String> {
        if self.local_udp_port > 0 {
            Some(format!("127.0.0.1:{}", self.local_udp_port))
        } else {
            None
        }
    }
    
    /// 获取统计信息
    pub fn get_stats(&self) -> BridgeStats {
        self.stats.to_stats()
    }
    
    /// 检查是否正在运行
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }
    
    /// 启动代理桥接（异步任务）
    pub async fn start(&self) -> Result<(), String> {
        if self.running.load(Ordering::Relaxed) {
            return Ok(()); // 已在运行
        }
        
        // 1. 启动 TCP 监听（HTTP CONNECT 代理）
        let addr = SocketAddr::from(([127, 0, 0, 1], self.local_port));
        let listener = TcpListener::bind(addr).await
            .map_err(|e| format!("绑定 TCP 端口 {} 失败: {}", self.local_port, e))?;
        
        self.running.store(true, Ordering::Relaxed);
        
        info!(
            profile_id = %self.profile_id,
            local_port = self.local_port,
            local_udp_port = self.local_udp_port,
            upstream = %format!("{}:{}", self.config.upstream_host, self.config.upstream_port),
            udp_enabled = self.config.enable_udp,
            "代理桥接已启动"
        );
        
        let running = Arc::clone(&self.running);
        let config = self.config.clone();
        let stats = Arc::clone(&self.stats);
        let profile_id = self.profile_id.clone();
        
        // 2. 启动 TCP 监听循环
        tokio::spawn(async move {
            while running.load(Ordering::Relaxed) {
                tokio::select! {
                    result = listener.accept() => {
                        match result {
                            Ok((client_stream, client_addr)) => {
                                debug!(
                                    profile_id = %profile_id,
                                    client = %client_addr,
                                    "新 TCP 连接"
                                );
                                
                                stats.total_connections.fetch_add(1, Ordering::Relaxed);
                                stats.active_connections.fetch_add(1, Ordering::Relaxed);
                                
                                let config = config.clone();
                                let stats = Arc::clone(&stats);
                                let profile_id = profile_id.clone();
                                
                                tokio::spawn(async move {
                                    if let Err(e) = handle_connection(
                                        client_stream, 
                                        &config,
                                        &stats,
                                    ).await {
                                        debug!(
                                            profile_id = %profile_id,
                                            error = %e,
                                            "TCP 连接处理失败"
                                        );
                                        stats.failed_connections.fetch_add(1, Ordering::Relaxed);
                                    }
                                    stats.active_connections.fetch_sub(1, Ordering::Relaxed);
                                });
                            }
                            Err(e) => {
                                error!(
                                    profile_id = %profile_id,
                                    error = %e,
                                    "接受 TCP 连接失败"
                                );
                                break;
                            }
                        }
                    }
                    _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {
                        // 定期检查是否应该停止
                        if !running.load(Ordering::Relaxed) {
                            break;
                        }
                    }
                }
            }
            
            info!(profile_id = %profile_id, "TCP 代理桥接监听循环已退出");
        });
        
        // 3. 如果启用 UDP，启动 UDP 转发（用于 WebRTC）
        if self.config.enable_udp && self.local_udp_port > 0 {
            self.start_udp_relay().await?;
        }
        
        Ok(())
    }
    
    /// 启动 UDP 中继（SOCKS5 UDP ASSOCIATE）
    async fn start_udp_relay(&self) -> Result<(), String> {
        let udp_addr = SocketAddr::from(([127, 0, 0, 1], self.local_udp_port));
        let local_socket = UdpSocket::bind(udp_addr).await
            .map_err(|e| format!("绑定 UDP 端口 {} 失败: {}", self.local_udp_port, e))?;
        
        info!(
            profile_id = %self.profile_id,
            udp_port = self.local_udp_port,
            "UDP 中继已启动"
        );
        
        let running = Arc::clone(&self.running);
        let config = self.config.clone();
        let stats = Arc::clone(&self.stats);
        let profile_id = self.profile_id.clone();
        
        tokio::spawn(async move {
            // 建立 SOCKS5 UDP ASSOCIATE 连接
            match establish_udp_associate(&config).await {
                Ok((control_stream, relay_addr)) => {
                    info!(
                        profile_id = %profile_id,
                        relay_addr = %relay_addr,
                        "SOCKS5 UDP ASSOCIATE 建立成功"
                    );
                    
                    // 创建到中继服务器的 UDP 套接字
                    let relay_socket = match UdpSocket::bind("0.0.0.0:0").await {
                        Ok(s) => s,
                        Err(e) => {
                            error!(profile_id = %profile_id, error = %e, "创建 UDP 中继套接字失败");
                            return;
                        }
                    };
                    
                    // 连接到中继地址
                    if let Err(e) = relay_socket.connect(&relay_addr).await {
                        error!(profile_id = %profile_id, error = %e, "连接 UDP 中继失败");
                        return;
                    }
                    
                    // UDP 转发循环 - 使用两个独立缓冲区避免借用冲突
                    let local_socket = Arc::new(local_socket);
                    let relay_socket = Arc::new(relay_socket);
                    let mut client_addr: Option<SocketAddr> = None;
                    
                    let mut recv_buf = vec![0u8; 65535];  // 接收客户端数据
                    let mut relay_buf = vec![0u8; 65535]; // 接收中继响应
                    
                    while running.load(Ordering::Relaxed) {
                        tokio::select! {
                            // 从客户端（浏览器）接收 UDP 数据
                            result = local_socket.recv_from(&mut recv_buf) => {
                                match result {
                                    Ok((n, addr)) => {
                                        client_addr = Some(addr);
                                        
                                        // 封装为 SOCKS5 UDP 数据包格式
                                        // [RSV(2) | FRAG(1) | ATYP(1) | DST.ADDR | DST.PORT | DATA]
                                        // 这里简化处理，直接转发（需要从数据包中解析目标地址）
                                        if let Err(e) = relay_socket.send(&recv_buf[..n]).await {
                                            debug!(profile_id = %profile_id, error = %e, "发送 UDP 数据失败");
                                        } else {
                                            stats.udp_packets_sent.fetch_add(1, Ordering::Relaxed);
                                        }
                                    }
                                    Err(e) => {
                                        debug!(profile_id = %profile_id, error = %e, "接收 UDP 数据失败");
                                    }
                                }
                            }
                            // 从中继服务器接收 UDP 响应
                            result = relay_socket.recv(&mut relay_buf) => {
                                match result {
                                    Ok(n) => {
                                        if let Some(addr) = client_addr {
                                            if let Err(e) = local_socket.send_to(&relay_buf[..n], addr).await {
                                                debug!(profile_id = %profile_id, error = %e, "转发 UDP 响应失败");
                                            } else {
                                                stats.udp_packets_received.fetch_add(1, Ordering::Relaxed);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        debug!(profile_id = %profile_id, error = %e, "接收 UDP 响应失败");
                                    }
                                }
                            }
                            _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {
                                if !running.load(Ordering::Relaxed) {
                                    break;
                                }
                            }
                        }
                    }
                    
                    // 保持控制连接存活（SOCKS5 规范要求）
                    drop(control_stream);
                }
                Err(e) => {
                    error!(
                        profile_id = %profile_id,
                        error = %e,
                        "SOCKS5 UDP ASSOCIATE 建立失败"
                    );
                }
            }
            
            info!(profile_id = %profile_id, "UDP 中继已退出");
        });
        
        Ok(())
    }
    
    /// 停止代理桥接
    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
        info!(
            profile_id = %self.profile_id,
            local_port = self.local_port,
            "代理桥接已停止"
        );
    }
}

/// 处理单个连接
async fn handle_connection(
    mut client: TcpStream,
    config: &ProxyBridgeConfig,
    stats: &BridgeStatsInner,
) -> Result<(), String> {
    // 1. 解析 HTTP CONNECT 请求
    let target = parse_http_connect(&mut client).await?;
    
    // 2. 连接上游代理
    let mut upstream = match config.upstream_type.as_str() {
        "socks5" => connect_socks5_proxy(
            &config.upstream_host,
            config.upstream_port,
            config.username.as_deref(),
            config.password.as_deref(),
            &target.0,
            target.1,
        ).await?,
        "http" | "https" => connect_http_proxy(
            &config.upstream_host,
            config.upstream_port,
            config.username.as_deref(),
            config.password.as_deref(),
            &target.0,
            target.1,
        ).await?,
        _ => return Err(format!("不支持的代理类型: {}", config.upstream_type)),
    };
    
    // 3. 发送 HTTP 200 响应给客户端
    client.write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n").await
        .map_err(|e| format!("发送响应失败: {}", e))?;
    
    // 4. 双向流量转发
    let (bytes_sent, bytes_received) = copy_bidirectional(&mut client, &mut upstream).await?;
    
    stats.bytes_sent.fetch_add(bytes_sent, Ordering::Relaxed);
    stats.bytes_received.fetch_add(bytes_received, Ordering::Relaxed);
    
    Ok(())
}

/// 解析 HTTP CONNECT 请求
async fn parse_http_connect(stream: &mut TcpStream) -> Result<(String, u16), String> {
    let mut buf = vec![0u8; 4096];
    let n = stream.read(&mut buf).await
        .map_err(|e| format!("读取请求失败: {}", e))?;
    
    if n == 0 {
        return Err("连接已关闭".to_string());
    }
    
    let request = String::from_utf8_lossy(&buf[..n]);
    debug!(request = %request, "收到 HTTP 请求");
    
    // 解析 "CONNECT example.com:443 HTTP/1.1"
    let first_line = request.lines().next()
        .ok_or("空请求")?;
    
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(format!("无效的请求格式: {}", first_line));
    }
    
    let method = parts[0].to_uppercase();
    if method != "CONNECT" {
        return Err(format!("不支持的方法: {}", method));
    }
    
    let target = parts[1];
    let (host, port) = parse_host_port(target)?;
    
    Ok((host, port))
}

/// 解析 host:port 格式
fn parse_host_port(target: &str) -> Result<(String, u16), String> {
    // 支持 [IPv6]:port 格式
    if target.starts_with('[') {
        if let Some(bracket_end) = target.find(']') {
            let host = target[1..bracket_end].to_string();
            let port_str = &target[bracket_end + 1..];
            let port = if port_str.starts_with(':') {
                port_str[1..].parse::<u16>()
                    .map_err(|_| format!("无效的端口: {}", port_str))?
            } else {
                443 // 默认 HTTPS 端口
            };
            return Ok((host, port));
        }
    }
    
    // 普通 host:port 格式
    if let Some(colon_pos) = target.rfind(':') {
        let host = target[..colon_pos].to_string();
        let port = target[colon_pos + 1..].parse::<u16>()
            .map_err(|_| format!("无效的端口: {}", &target[colon_pos + 1..]))?;
        Ok((host, port))
    } else {
        Ok((target.to_string(), 443)) // 默认 HTTPS 端口
    }
}

/// 连接 SOCKS5 代理（含认证）
async fn connect_socks5_proxy(
    proxy_host: &str,
    proxy_port: u16,
    username: Option<&str>,
    password: Option<&str>,
    target_host: &str,
    target_port: u16,
) -> Result<TcpStream, String> {
    // 1. 连接到 SOCKS5 代理服务器
    let proxy_addr = format!("{}:{}", proxy_host, proxy_port);
    let mut stream = TcpStream::connect(&proxy_addr).await
        .map_err(|e| format!("连接 SOCKS5 代理失败: {}", e))?;
    
    // 2. SOCKS5 握手
    let auth_required = username.is_some() && password.is_some();
    
    // 发送支持的认证方法
    if auth_required {
        // 0x02: 用户名/密码认证
        stream.write_all(&[0x05, 0x02, 0x00, 0x02]).await
            .map_err(|e| format!("发送认证方法失败: {}", e))?;
    } else {
        // 0x00: 无认证
        stream.write_all(&[0x05, 0x01, 0x00]).await
            .map_err(|e| format!("发送认证方法失败: {}", e))?;
    }
    
    // 读取服务器选择的认证方法
    let mut response = [0u8; 2];
    stream.read_exact(&mut response).await
        .map_err(|e| format!("读取认证响应失败: {}", e))?;
    
    if response[0] != 0x05 {
        return Err(format!("无效的 SOCKS 版本: {}", response[0]));
    }
    
    // 3. 如果需要用户名/密码认证
    if response[1] == 0x02 {
        let user = username.ok_or("需要用户名")?;
        let pass = password.ok_or("需要密码")?;
        
        // 构建认证请求: [版本, 用户名长度, 用户名, 密码长度, 密码]
        let mut auth_request = vec![0x01]; // 认证子协议版本
        auth_request.push(user.len() as u8);
        auth_request.extend(user.as_bytes());
        auth_request.push(pass.len() as u8);
        auth_request.extend(pass.as_bytes());
        
        stream.write_all(&auth_request).await
            .map_err(|e| format!("发送认证请求失败: {}", e))?;
        
        // 读取认证结果
        let mut auth_response = [0u8; 2];
        stream.read_exact(&mut auth_response).await
            .map_err(|e| format!("读取认证结果失败: {}", e))?;
        
        if auth_response[1] != 0x00 {
            return Err("SOCKS5 认证失败".to_string());
        }
        
        debug!("SOCKS5 认证成功");
    } else if response[1] == 0xFF {
        return Err("SOCKS5 服务器不支持所提供的认证方法".to_string());
    }
    
    // 4. 发送 CONNECT 请求
    let mut connect_request = vec![
        0x05, // SOCKS 版本
        0x01, // CONNECT 命令
        0x00, // 保留字节
    ];
    
    // 地址类型和目标地址
    if target_host.parse::<std::net::Ipv4Addr>().is_ok() {
        // IPv4 地址
        connect_request.push(0x01);
        for octet in target_host.split('.') {
            connect_request.push(octet.parse::<u8>().unwrap());
        }
    } else if target_host.parse::<std::net::Ipv6Addr>().is_ok() {
        // IPv6 地址
        connect_request.push(0x04);
        let addr: std::net::Ipv6Addr = target_host.parse().unwrap();
        connect_request.extend(addr.octets());
    } else {
        // 域名
        connect_request.push(0x03);
        connect_request.push(target_host.len() as u8);
        connect_request.extend(target_host.as_bytes());
    }
    
    // 端口（大端序）
    connect_request.push((target_port >> 8) as u8);
    connect_request.push((target_port & 0xFF) as u8);
    
    stream.write_all(&connect_request).await
        .map_err(|e| format!("发送 CONNECT 请求失败: {}", e))?;
    
    // 5. 读取 CONNECT 响应
    let mut connect_response = [0u8; 10];
    stream.read_exact(&mut connect_response[..4]).await
        .map_err(|e| format!("读取 CONNECT 响应失败: {}", e))?;
    
    if connect_response[1] != 0x00 {
        let error_msg = match connect_response[1] {
            0x01 => "一般 SOCKS 服务器故障",
            0x02 => "规则不允许连接",
            0x03 => "网络不可达",
            0x04 => "主机不可达",
            0x05 => "连接被拒绝",
            0x06 => "TTL 过期",
            0x07 => "不支持的命令",
            0x08 => "不支持的地址类型",
            _ => "未知错误",
        };
        return Err(format!("SOCKS5 CONNECT 失败: {}", error_msg));
    }
    
    // 读取剩余的响应（绑定地址和端口）
    let addr_type = connect_response[3];
    match addr_type {
        0x01 => {
            // IPv4
            let mut buf = [0u8; 6];
            stream.read_exact(&mut buf).await.ok();
        }
        0x03 => {
            // 域名
            let mut len_buf = [0u8; 1];
            stream.read_exact(&mut len_buf).await.ok();
            let mut domain_buf = vec![0u8; len_buf[0] as usize + 2];
            stream.read_exact(&mut domain_buf).await.ok();
        }
        0x04 => {
            // IPv6
            let mut buf = [0u8; 18];
            stream.read_exact(&mut buf).await.ok();
        }
        _ => {}
    }
    
    debug!(
        target = %format!("{}:{}", target_host, target_port),
        "SOCKS5 连接建立成功"
    );
    
    Ok(stream)
}

/// 连接 HTTP/HTTPS 代理
async fn connect_http_proxy(
    proxy_host: &str,
    proxy_port: u16,
    username: Option<&str>,
    password: Option<&str>,
    target_host: &str,
    target_port: u16,
) -> Result<TcpStream, String> {
    // 连接到 HTTP 代理服务器
    let proxy_addr = format!("{}:{}", proxy_host, proxy_port);
    let mut stream = TcpStream::connect(&proxy_addr).await
        .map_err(|e| format!("连接 HTTP 代理失败: {}", e))?;
    
    // 构建 CONNECT 请求
    let mut request = format!(
        "CONNECT {}:{} HTTP/1.1\r\nHost: {}:{}\r\n",
        target_host, target_port, target_host, target_port
    );
    
    // 添加代理认证
    if let (Some(user), Some(pass)) = (username, password) {
        use base64::Engine;
        let credentials = format!("{}:{}", user, pass);
        let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);
        request.push_str(&format!("Proxy-Authorization: Basic {}\r\n", encoded));
    }
    
    request.push_str("\r\n");
    
    stream.write_all(request.as_bytes()).await
        .map_err(|e| format!("发送 CONNECT 请求失败: {}", e))?;
    
    // 读取响应
    let mut buf = vec![0u8; 4096];
    let n = stream.read(&mut buf).await
        .map_err(|e| format!("读取响应失败: {}", e))?;
    
    let response = String::from_utf8_lossy(&buf[..n]);
    
    // 检查状态码
    if !response.contains("200") {
        return Err(format!("HTTP 代理连接失败: {}", response.lines().next().unwrap_or("")));
    }
    
    debug!(
        target = %format!("{}:{}", target_host, target_port),
        "HTTP 代理连接建立成功"
    );
    
    Ok(stream)
}

/// 双向流量转发
async fn copy_bidirectional(
    client: &mut TcpStream,
    server: &mut TcpStream,
) -> Result<(u64, u64), String> {
    let (mut client_reader, mut client_writer) = client.split();
    let (mut server_reader, mut server_writer) = server.split();
    
    let client_to_server = tokio::io::copy(&mut client_reader, &mut server_writer);
    let server_to_client = tokio::io::copy(&mut server_reader, &mut client_writer);
    
    let result = tokio::try_join!(client_to_server, server_to_client)
        .map_err(|e| format!("流量转发失败: {}", e))?;
    
    Ok(result)
}

/// 建立 SOCKS5 UDP ASSOCIATE 连接
/// 返回控制连接和 UDP 中继地址
async fn establish_udp_associate(config: &ProxyBridgeConfig) -> Result<(TcpStream, String), String> {
    // 1. 连接到 SOCKS5 代理服务器
    let proxy_addr = format!("{}:{}", config.upstream_host, config.upstream_port);
    let mut stream = TcpStream::connect(&proxy_addr).await
        .map_err(|e| format!("连接 SOCKS5 代理失败: {}", e))?;
    
    // 2. SOCKS5 握手
    let auth_required = config.username.is_some() && config.password.is_some();
    
    if auth_required {
        stream.write_all(&[0x05, 0x02, 0x00, 0x02]).await
            .map_err(|e| format!("发送认证方法失败: {}", e))?;
    } else {
        stream.write_all(&[0x05, 0x01, 0x00]).await
            .map_err(|e| format!("发送认证方法失败: {}", e))?;
    }
    
    let mut response = [0u8; 2];
    stream.read_exact(&mut response).await
        .map_err(|e| format!("读取认证响应失败: {}", e))?;
    
    if response[0] != 0x05 {
        return Err(format!("无效的 SOCKS 版本: {}", response[0]));
    }
    
    // 3. 如果需要用户名/密码认证
    if response[1] == 0x02 {
        let user = config.username.as_deref().ok_or("需要用户名")?;
        let pass = config.password.as_deref().ok_or("需要密码")?;
        
        let mut auth_request = vec![0x01];
        auth_request.push(user.len() as u8);
        auth_request.extend(user.as_bytes());
        auth_request.push(pass.len() as u8);
        auth_request.extend(pass.as_bytes());
        
        stream.write_all(&auth_request).await
            .map_err(|e| format!("发送认证请求失败: {}", e))?;
        
        let mut auth_response = [0u8; 2];
        stream.read_exact(&mut auth_response).await
            .map_err(|e| format!("读取认证结果失败: {}", e))?;
        
        if auth_response[1] != 0x00 {
            return Err("SOCKS5 认证失败".to_string());
        }
        
        debug!("SOCKS5 UDP ASSOCIATE 认证成功");
    } else if response[1] == 0xFF {
        return Err("SOCKS5 服务器不支持所提供的认证方法".to_string());
    }
    
    // 4. 发送 UDP ASSOCIATE 请求 (CMD = 0x03)
    // DST.ADDR 和 DST.PORT 设为 0，表示客户端会从任意地址发送
    let udp_associate_request = vec![
        0x05, // SOCKS 版本
        0x03, // UDP ASSOCIATE 命令
        0x00, // 保留字节
        0x01, // 地址类型: IPv4
        0x00, 0x00, 0x00, 0x00, // 地址: 0.0.0.0
        0x00, 0x00, // 端口: 0
    ];
    
    stream.write_all(&udp_associate_request).await
        .map_err(|e| format!("发送 UDP ASSOCIATE 请求失败: {}", e))?;
    
    // 5. 读取响应
    let mut response_header = [0u8; 4];
    stream.read_exact(&mut response_header).await
        .map_err(|e| format!("读取 UDP ASSOCIATE 响应失败: {}", e))?;
    
    if response_header[1] != 0x00 {
        let error_msg = match response_header[1] {
            0x01 => "一般 SOCKS 服务器故障",
            0x02 => "规则不允许连接",
            0x03 => "网络不可达",
            0x04 => "主机不可达",
            0x05 => "连接被拒绝",
            0x06 => "TTL 过期",
            0x07 => "不支持的命令",
            0x08 => "不支持的地址类型",
            _ => "未知错误",
        };
        return Err(format!("SOCKS5 UDP ASSOCIATE 失败: {}", error_msg));
    }
    
    // 6. 解析 BND.ADDR 和 BND.PORT（UDP 中继地址）
    let relay_addr = match response_header[3] {
        0x01 => {
            // IPv4
            let mut addr_buf = [0u8; 6];
            stream.read_exact(&mut addr_buf).await
                .map_err(|e| format!("读取中继地址失败: {}", e))?;
            
            let ip = format!("{}.{}.{}.{}", addr_buf[0], addr_buf[1], addr_buf[2], addr_buf[3]);
            let port = ((addr_buf[4] as u16) << 8) | (addr_buf[5] as u16);
            
            // 如果服务器返回 0.0.0.0，使用代理服务器地址
            if ip == "0.0.0.0" {
                format!("{}:{}", config.upstream_host, port)
            } else {
                format!("{}:{}", ip, port)
            }
        }
        0x03 => {
            // 域名
            let mut len_buf = [0u8; 1];
            stream.read_exact(&mut len_buf).await
                .map_err(|e| format!("读取域名长度失败: {}", e))?;
            
            let mut domain_buf = vec![0u8; len_buf[0] as usize];
            stream.read_exact(&mut domain_buf).await
                .map_err(|e| format!("读取域名失败: {}", e))?;
            
            let mut port_buf = [0u8; 2];
            stream.read_exact(&mut port_buf).await
                .map_err(|e| format!("读取端口失败: {}", e))?;
            
            let domain = String::from_utf8_lossy(&domain_buf).to_string();
            let port = ((port_buf[0] as u16) << 8) | (port_buf[1] as u16);
            
            format!("{}:{}", domain, port)
        }
        0x04 => {
            // IPv6
            let mut addr_buf = [0u8; 18];
            stream.read_exact(&mut addr_buf).await
                .map_err(|e| format!("读取 IPv6 地址失败: {}", e))?;
            
            let addr: std::net::Ipv6Addr = std::net::Ipv6Addr::from([
                addr_buf[0], addr_buf[1], addr_buf[2], addr_buf[3],
                addr_buf[4], addr_buf[5], addr_buf[6], addr_buf[7],
                addr_buf[8], addr_buf[9], addr_buf[10], addr_buf[11],
                addr_buf[12], addr_buf[13], addr_buf[14], addr_buf[15],
            ]);
            let port = ((addr_buf[16] as u16) << 8) | (addr_buf[17] as u16);
            
            format!("[{}]:{}", addr, port)
        }
        _ => {
            return Err(format!("不支持的地址类型: {}", response_header[3]));
        }
    };
    
    debug!(relay_addr = %relay_addr, "UDP 中继地址解析成功");
    
    // 返回控制连接和中继地址
    // 注意：必须保持控制连接存活，否则 UDP ASSOCIATE 会失效
    Ok((stream, relay_addr))
}

// ==================== 代理桥接管理器 ====================

/// 代理桥接管理器（管理所有 Profile 的桥接实例）
pub struct ProxyBridgeManager {
    bridges: Arc<Mutex<HashMap<String, Arc<ProxyBridge>>>>,
}

impl ProxyBridgeManager {
    pub fn new() -> Self {
        Self {
            bridges: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// 为 Profile 启动代理桥接
    pub async fn start_bridge(
        &self,
        profile_id: &str,
        config: ProxyBridgeConfig,
    ) -> Result<String, String> {
        let mut bridges = self.bridges.lock().await;
        
        // 检查是否已存在
        if let Some(bridge) = bridges.get(profile_id) {
            if bridge.is_running() {
                return Ok(bridge.local_addr());
            }
        }
        
        // 创建新的桥接实例
        let bridge = Arc::new(ProxyBridge::new(profile_id.to_string(), config));
        bridge.start().await?;
        
        let local_addr = bridge.local_addr();
        bridges.insert(profile_id.to_string(), bridge);
        
        Ok(local_addr)
    }
    
    /// 停止 Profile 的代理桥接
    pub async fn stop_bridge(&self, profile_id: &str) -> Result<(), String> {
        let mut bridges = self.bridges.lock().await;
        
        if let Some(bridge) = bridges.remove(profile_id) {
            bridge.stop();
        }
        
        Ok(())
    }
    
    /// 获取桥接统计信息
    pub async fn get_bridge_stats(&self, profile_id: &str) -> Option<BridgeStats> {
        let bridges = self.bridges.lock().await;
        bridges.get(profile_id).map(|b| b.get_stats())
    }
    
    /// 获取所有活跃桥接的统计信息
    pub async fn get_all_stats(&self) -> HashMap<String, BridgeStats> {
        let bridges = self.bridges.lock().await;
        bridges
            .iter()
            .map(|(id, bridge)| (id.clone(), bridge.get_stats()))
            .collect()
    }
    
    /// 停止所有代理桥接
    pub async fn stop_all(&self) {
        let mut bridges = self.bridges.lock().await;
        for (_, bridge) in bridges.drain() {
            bridge.stop();
        }
    }
    
    /// 检查是否需要桥接（带密码的 SOCKS5）
    pub fn needs_bridge(proxy_type: &str, username: &Option<String>, password: &Option<String>) -> bool {
        proxy_type.to_lowercase() == "socks5" 
            && (username.is_some() || password.is_some())
    }
}

impl Default for ProxyBridgeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_host_port() {
        assert_eq!(parse_host_port("example.com:443").unwrap(), ("example.com".to_string(), 443));
        assert_eq!(parse_host_port("example.com:8080").unwrap(), ("example.com".to_string(), 8080));
        assert_eq!(parse_host_port("192.168.1.1:80").unwrap(), ("192.168.1.1".to_string(), 80));
        assert_eq!(parse_host_port("[::1]:443").unwrap(), ("::1".to_string(), 443));
    }
    
    #[test]
    fn test_needs_bridge() {
        assert!(ProxyBridgeManager::needs_bridge("socks5", &Some("user".to_string()), &Some("pass".to_string())));
        assert!(ProxyBridgeManager::needs_bridge("SOCKS5", &Some("user".to_string()), &None));
        assert!(!ProxyBridgeManager::needs_bridge("socks5", &None, &None));
        assert!(!ProxyBridgeManager::needs_bridge("http", &Some("user".to_string()), &Some("pass".to_string())));
    }
    
    #[test]
    fn test_find_free_port() {
        let port = ProxyBridge::find_free_port(50000);
        assert!(port >= 50000 || port == 0);
    }
}
