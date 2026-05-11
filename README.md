# 传媒技术学院 2026 届毕业设计展网站

> 智媒融合 · 创启未来

## 项目概述

本项目是传媒技术学院 2026 届毕业设计展的官方网站，展示软件工程、电子信息工程、广播电视工程三大专业的优秀毕业设计作品。

### 技术栈

**前端**:
- Nuxt 3 (Vue 3 SSR 框架)
- Tailwind CSS (样式框架)
- Pinia (状态管理)
- Three.js (3D 效果)
- GSAP (动画库)
- TypeScript

**后端**:
- Rust + axum 0.7 (Web 框架)
- MySQL 8.0 (数据库)
- Redis (缓存和限流)
- SQLx (数据库驱动)

**部署**:
- Docker + Docker Compose

## 📚 文档导航

- **[CLAUDE.md](CLAUDE.md)** — 完整项目文档（开发指南）
- **[PRD.md](PRD.md)** — 产品需求文档
- **[DEPLOYMENT.md](DEPLOYMENT.md)** — 部署指南
- **[COS_MIGRATION.md](COS_MIGRATION.md)** — COS 资源迁移说明

## 📦 静态资源说明

作品视频和封面存储在**腾讯云 COS 对象存储**，无需本地存放静态文件。

**COS 基础 URL**: `https://whcm-1353140174.cos.ap-nanjing.myqcloud.com`

### 资源路径
- 软件工程: `resource/Se/001-012.mp4`（12个）
- 电子信息工程: `resource/Ele/001-011.mp4`（11个）
- 广播电视工程: `resource/Rte/001-010.mp4`（10个）

**总计**: 33个作品视频，详情参见 [COS_MIGRATION.md](COS_MIGRATION.md)

## 🚀 快速启动

### 前置准备

1. **克隆项目**：
   ```bash
   git clone <仓库地址>
   cd 传媒技术学院2026界毕业设计展网页
   ```

2. **静态资源**：
   - 视频和封面存储在腾讯云 COS，无需本地存放
   - 详见 [COS_MIGRATION.md](COS_MIGRATION.md)

3. **安装依赖**：
   - **Docker Desktop**（推荐，用于 MySQL 和 Redis）
   - 或本地安装 MySQL 8.0 + Redis 7

### 一键启动（推荐）

**Windows 用户**：
```cmd
双击 start.bat
```

**Linux/Mac 用户**：
```bash
./start.sh
```

启动完成后访问：http://localhost:3000

### 手动启动

详见 [DEPLOYMENT.md](DEPLOYMENT.md)

## 项目结构

```
.
├── frontend/              # Nuxt 3 前端项目
│   ├── pages/            # 页面路由（6个）
│   ├── components/       # Vue 组件（7个）
│   ├── stores/           # Pinia 状态管理（3个）
│   ├── composables/      # 组合式函数
│   ├── types/            # TypeScript 类型定义
│   └── assets/           # 静态资源
├── backend/              # Rust axum 后端项目
│   ├── src/
│   │   ├── routes/      # API 路由（6个模块）
│   │   ├── models/      # 数据模型（4个）
│   │   ├── services/    # 业务逻辑（3个）
│   │   └── utils/       # 工具函数（2个）
│   ├── migrations/      # 数据库迁移脚本
│   └── backend/         # SQL 导入脚本
│       ├── import_works.sql           # 作品数据导入
│       └── import_works_complete.sql  # 完整33作品导入
├── docker-compose.yml    # Docker 编排配置
├── start.sh / start.bat  # 一键启动脚本
└── README.md
```

## 核心功能

### 页面路由（6个）

- `/` - 首页（3D 立方体、专业入口）
- `/majors` - 专业展区
- `/major/:majorCode` - 专业作品列表（software/electronic/broadcast）
- `/works/:workId` - 作品详情（视频播放、点赞、评论）
- `/ranking` - 点赞热榜
- `/about` - 关于毕展

### API 接口（10+）

- `GET /api/health` - 健康检查
- `GET /api/works` - 获取作品列表（支持分页和专业筛选）
- `GET /api/works/:id` - 获取作品详情
- `GET /api/interactions/summary` - 批量获取交互统计
- `GET /api/works/:workId/interaction` - 获取单个作品交互状态
- `POST /api/works/:workId/like/toggle` - 切换点赞状态
- `GET /api/works/:workId/comments` - 获取评论列表（游标分页）
- `POST /api/works/:workId/comments` - 提交评论
- `GET /api/rankings/likes?range=all` - 获取点赞热榜（all/today/week）

### 核心特性

**游客身份识别**：
- 前端：localStorage 存储 visitor_id（UUID）
- 后端：SHA256(work_id + visitor_id + ip + server_salt) 生成指纹哈希
- 自动生成中文昵称：形容词+名词+4位数字（如"快乐熊猫4821"）

**限流策略**（Redis 滑动窗口）：
- 点赞：10秒/次（同一指纹对同一作品）
- 评论：30秒/次（同一指纹）
- IP：60秒/20次（同一IP）

**安全防护**：
- XSS 防护：HTML 转义
- SQL 注入防护：SQLx 参数化查询
- 蜜罐字段：website 字段检测机器人
- 敏感词过滤：本地敏感词库
- IP 和指纹：SHA256 哈希后存储

## 数据库设计

### works 表
作品完整信息（id, title, major_code, tags, authors, teacher, poster_url, video_url, introduction）

### likes 表
点赞记录（软删除，is_active 字段）

### comments 表
评论记录（游标分页，status 字段）

### abuse_logs 表
风控日志

## 环境要求

### 开发环境
- Docker Desktop（必须）
- Node.js >= 18
- Rust >= 1.70
- pnpm >= 9.0（前端必须使用 pnpm）

### 端口占用
- 3000：前端
- 8080：后端
- 3306：MySQL
- 6379：Redis

## 配置说明

### 前端环境变量 (`frontend/.env`)

```env
NUXT_PUBLIC_API_BASE=http://localhost:8080/api
NUXT_PUBLIC_COS_BASE_URL=https://whcm-1353140174.cos.ap-nanjing.myqcloud.com
```

### 后端环境变量 (`backend/.env`)

```env
APP_HOST=0.0.0.0
APP_PORT=8080
DATABASE_URL=mysql://root:123456@localhost:3306/graduation_exhibition
REDIS_URL=redis://localhost:6379
SERVER_SALT=your-secret-salt-here  # 生产环境必须修改
CORS_ALLOWED_ORIGINS=http://localhost:3000
TRUST_PROXY=true
COS_BASE_URL=https://whcm-1353140174.cos.ap-nanjing.myqcloud.com
```

## 开发命令

### 前端开发

```bash
cd frontend
pnpm install          # 安装依赖（必须使用 pnpm）
pnpm dev              # 启动开发服务器 (http://localhost:3000)
pnpm build            # 构建生产版本
pnpm preview          # 预览生产构建
```

### 后端开发

```bash
cd backend
cargo build           # 构建项目
cargo run             # 运行开发服务器 (http://localhost:8080)
cargo test            # 运行测试
cargo build --release # 构建生产版本
cargo fmt             # 格式化代码
cargo clippy          # 代码检查
```

### Docker 管理

```bash
docker-compose up -d        # 启动 MySQL + Redis
docker-compose logs -f      # 查看日志
docker-compose down         # 停止服务
docker-compose down -v      # 停止并删除数据卷
```

## 项目亮点

### 技术亮点
1. **Rust 后端**：高性能、类型安全、内存安全
2. **Nuxt 3 SSR**：首屏加载快、SEO 友好
3. **Redis 限流**：滑动窗口算法，精确控制
4. **游标分页**：避免深分页性能问题
5. **软删除**：数据可恢复，用户体验好

### 安全亮点
1. **多层防护**：指纹识别 + 限流 + 内容审核
2. **无明文存储**：IP 和指纹全部哈希
3. **防刷机制**：蜜罐字段 + 限流 + 敏感词
4. **XSS 防护**：HTML 转义
5. **SQL 注入防护**：参数化查询

### 用户体验
1. **无需登录**：游客直接使用
2. **自动昵称**：随机生成中文昵称
3. **实时反馈**：点赞、评论即时更新
4. **流畅动画**：GSAP 专业级动画
5. **响应式设计**：全设备适配

## 常见问题

详见 [DEPLOYMENT.md](DEPLOYMENT.md) 的"常见问题"章节

## 许可证

© 2026 传媒技术学院. All rights reserved.
