/**
 * @file mock.data.ts
 * @description 环境列表 Mock 数据 (开发调试用)
 */

import type { Profile, ProfileStatus } from '@/types'

const platforms = ['windows', 'macos', 'linux'] as const
const brands = ['Chrome', 'Edge', 'Brave']
const timezones = ['Asia/Shanghai', 'America/New_York', 'Europe/London', 'Asia/Tokyo']
const statuses: ProfileStatus[] = ['stopped', 'running', 'stopped', 'stopped', 'running']

/**
 * 生成随机 Mock Profile
 */
function generateMockProfile(index: number): Profile {
    const id = `profile_${String(index).padStart(4, '0')}`
    const status = statuses[index % statuses.length]

    return {
        id,
        name: `环境 ${index + 1}`,
        group: index % 3 === 0 ? '默认分组' : index % 3 === 1 ? '电商账号' : '社交媒体',
        status,
        dataDir: `D:\\chubao-cache\\${id}`,
        cdpPort: status === 'running' ? 9222 + index : undefined,
        fingerprint: {
            seed: Date.now() + index,
            platform: platforms[index % 3],
            browser: ['chrome', 'edge', 'brave'][index % 3] as any,
            userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
            brand: brands[index % 3],
            version: '120.0.0.0',
            hardwareConcurrency: [4, 8, 12, 16][index % 4],
            deviceMemory: [4, 8, 16, 32][index % 4],
            screenResolution: `${[1920, 2560, 1440][index % 3]}x${[1080, 1440, 900][index % 3]}`,
            screenWidth: [1920, 2560, 1440][index % 3],
            screenHeight: [1080, 1440, 900][index % 3],
            timezone: timezones[index % 4],
            language: 'zh-CN',
            canvasNoise: true,
            webglNoise: true,
            audioNoise: true,
            webrtc: 'fake',
            webglVendor: 'Google Inc. (NVIDIA)',
            webglRenderer: 'ANGLE (NVIDIA, NVIDIA GeForce RTX 3080 Direct3D11 vs_5_0 ps_5_0, D3D11)',
            webgpu: true,
            canvas: 'noise',
            audioContext: 'noise',
            doNotTrack: 'unspecified',
            clientRects: true,
            mediaDevices: 'fake',
            portScanProtection: true,
            hardwareAcceleration: true,
            disableSandbox: false
        },
        proxy: index % 2 === 0 ? {
            type: 'socks5',
            host: `proxy${index % 5 + 1}.example.com`,
            port: 1080,
            username: 'user',
            password: '****',
        } : undefined,
        remark: index % 4 === 0 ? '重要账号' : undefined,
        createdAt: Date.now() - index * 86400000,
        updatedAt: Date.now() - index * 3600000,
    }
}

/**
 * 生成 Mock 环境列表
 */
export function generateMockProfiles(count: number = 50): Profile[] {
    return Array.from({ length: count }, (_, i) => generateMockProfile(i))
}

export const mockProfiles = generateMockProfiles(50)
