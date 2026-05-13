# 传媒技术学院 2026 届毕业设计展网站

> 智媒融合 · 创启未来

## 项目概述

传媒技术学院 2026 届毕业设计展官方网站，展示软件工程、电子信息工程、广播电视工程三个专业的 33 个优秀毕业设计作品。面向公众开放，游客可直接浏览、点赞、评论。

### 技术栈

| 层 | 技术 |
|---|---|
| 前端 | Nuxt 3 + Vue 3 + TypeScript + Tailwind CSS + Pinia + Three.js + GSAP |
| 后端 | Rust + axum 0.7 + SQLx + MySQL 8.0 |
| 缓存/限流 | Redis 7 |
| 静态资源 | 腾讯云 COS 对象存储 |
| 部署 | Nginx 静态托管 (SSG) + Rust API |

## 📚 文档导航

- **[CLAUDE.md](CLAUDE.md)** — 完整项目文档（AI 开发指南）
- **[DEPLOYMENT.md](DEPLOYMENT.md)** — 部署指南（含 Nginx 配置 + 数据库）
- **[COS_MIGRATION.md](COS_MIGRATION.md)** — COS 资源管理说明
- **[毕业设计作品汇总.md](毕业设计作品汇总.md)** — 33个作品源数据

## 🚀 快速启动

### 开发模式

```bash
# 1. 启动 MySQL 和 Redis
docker-compose up -d

# 2. 初始化数据库
mysql -uroot -p123456 -e "CREATE DATABASE IF NOT EXISTS graduation_exhibition CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"
for f in backend/migrations/*.sql; do mysql -uroot -p123456 --default-character-set=utf8mb4 graduation_exhibition < "$f"; done
mysql -uroot -p123456 --default-character-set=utf8mb4 graduation_exhibition < backend/backend/import_works.sql

# 3. 启动后端
cd backend && cargo run

# 4. 启动前端（新终端）
cd frontend && pnpm install && pnpm dev
```

访问 http://localhost:3000

### 生产模式（SSG + Nginx）

```bash
# 1. 构建静态文件（后端需运行中）
cd frontend && pnpm generate

# 2. 启动 Nginx 托管 .output/public/
nginx.exe
```

访问 http://localhost

## 页面路由

| 路由 | 说明 |
|------|------|
| `/` | 首页 |
| `/major/:majorCode` | 专业作品列表 |
| `/works/:workId` | 作品详情 |
| `/ranking` | 点赞热榜 |

## 核心特性

- **无需登录**：游客直接浏览、点赞、评论，自动生成昵称
- **智能防刷**：Redis 滑动窗口限流 + SHA256 指纹识别 + 敏感词过滤
- **安全防护**：XSS 转义、SQL 参数化查询、蜜罐字段、数据哈希存储
- **日重置点赞**：每天凌晨后同一用户可对同一作品再次点赞
- **响应式设计**：PC、平板、手机全适配

## 环境要求

- Node.js >= 18 + pnpm >= 9
- Rust >= 1.70
- MySQL 8.0
- Redis 7

## 端口占用

| 端口 | 服务 |
|------|------|
| 3000 | 前端 (Nuxt) |
| 8080 | 后端 (Rust) |
| 3306 | MySQL |
| 6379 | Redis |

## 许可证

© 2026 传媒技术学院. All rights reserved.
