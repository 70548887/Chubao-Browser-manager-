/**
 * 常用时区配置
 */

export interface TimezoneOption {
    id: string
    label: string
    offset: number // UTC偏移（分钟）
    region: string
}

export const COMMON_TIMEZONES: TimezoneOption[] = [
    // 亚洲
    { id: 'Asia/Shanghai', label: '中国标准时间 (UTC+8)', offset: -480, region: 'Asia' },
    { id: 'Asia/Hong_Kong', label: '香港 (UTC+8)', offset: -480, region: 'Asia' },
    { id: 'Asia/Tokyo', label: '日本 (UTC+9)', offset: -540, region: 'Asia' },
    { id: 'Asia/Seoul', label: '韩国 (UTC+9)', offset: -540, region: 'Asia' },
    { id: 'Asia/Singapore', label: '新加坡 (UTC+8)', offset: -480, region: 'Asia' },
    { id: 'Asia/Dubai', label: '迪拜 (UTC+4)', offset: -240, region: 'Asia' },

    // 美洲
    { id: 'America/New_York', label: '美国东部时间 (UTC-5)', offset: 300, region: 'America' },
    { id: 'America/Los_Angeles', label: '美国西部时间 (UTC-8)', offset: 480, region: 'America' },
    { id: 'America/Chicago', label: '美国中部时间 (UTC-6)', offset: 360, region: 'America' },

    // 欧洲
    { id: 'Europe/London', label: '英国 (UTC+0)', offset: 0, region: 'Europe' },
    { id: 'Europe/Paris', label: '法国/德国 (UTC+1)', offset: -60, region: 'Europe' },
    { id: 'Europe/Moscow', label: '俄罗斯莫斯科 (UTC+3)', offset: -180, region: 'Europe' },

    // 大洋洲
    { id: 'Australia/Sydney', label: '澳大利亚悉尼 (UTC+10)', offset: -600, region: 'Oceania' },

    // 其他
    { id: 'UTC', label: '协调世界时 (UTC+0)', offset: 0, region: 'Other' },
    { id: 'Pacific/Auckland', label: '新西兰 (UTC+12)', offset: -720, region: 'Oceania' },
]

/**
 * 根据时区ID获取偏移量
 */
export function getTimezoneOffset(timezoneId: string): number {
    const tz = COMMON_TIMEZONES.find(t => t.id === timezoneId)
    return tz?.offset ?? -480 // 默认 UTC+8
}

/**
 * 根据地区筛选时区
 */
export function getTimezonesByRegion(region: string): TimezoneOption[] {
    return COMMON_TIMEZONES.filter(tz => tz.region === region)
}
