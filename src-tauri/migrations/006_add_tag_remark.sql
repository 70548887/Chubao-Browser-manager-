-- Migration 006: Add remark field to tags table

-- 为 tags 表添加 remark 字段
ALTER TABLE tags ADD COLUMN remark TEXT;
