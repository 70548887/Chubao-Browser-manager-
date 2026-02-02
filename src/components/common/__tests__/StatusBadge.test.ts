/**
 * @file StatusBadge.test.ts
 * @description StatusBadge 组件单元测试
 * @author DeepAgent
 */

import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import StatusBadge from '../StatusBadge.vue'

describe('StatusBadge', () => {
    describe('渲染测试', () => {
        it('应该正确渲染运行中状态', () => {
            const wrapper = mount(StatusBadge, {
                props: {
                    type: 'success',
                    label: '运行中',
                    showDot: true,
                    pulse: true
                },
            })

            expect(wrapper.text()).toContain('运行中')
            expect(wrapper.classes()).toContain('status-success')
            expect(wrapper.find('.status-dot').exists()).toBe(true)
            expect(wrapper.find('.dot-pulse').exists()).toBe(true)
        })

        it('应该正确渲染已停止状态', () => {
            const wrapper = mount(StatusBadge, {
                props: {
                    type: 'neutral',
                    label: '已停止'
                },
            })

            expect(wrapper.text()).toContain('已停止')
            expect(wrapper.classes()).toContain('status-neutral')
        })

        it('应该正确渲染错误状态', () => {
            const wrapper = mount(StatusBadge, {
                props: {
                    type: 'danger',
                    label: '错误'
                },
            })

            expect(wrapper.text()).toContain('错误')
            expect(wrapper.classes()).toContain('status-danger')
        })
    })

    describe('Props 验证', () => {
        it('应该接受有效的 type prop', () => {
            const validTypes = ['success', 'warning', 'danger', 'info', 'neutral', 'primary']

            validTypes.forEach((type) => {
                const wrapper = mount(StatusBadge, {
                    props: { type: type as any },
                })
                expect(wrapper.exists()).toBe(true)
                expect(wrapper.classes()).toContain(`status-${type}`)
            })
        })
    })

    describe('样式测试', () => {
        it('不同状态应该有不同的样式类', () => {
            const successWrapper = mount(StatusBadge, {
                props: { type: 'success' },
            })
            const neutralWrapper = mount(StatusBadge, {
                props: { type: 'neutral' },
            })

            expect(successWrapper.classes()).not.toEqual(neutralWrapper.classes())
        })
    })
})
