# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

**传媒技术学院 2026 届毕业设计展网站** - 一个面向公众的毕业设计展览网站，展示三个专业的学生作品：软件工程、电子信息工程、广播电视工程。

**主题**：智媒融合 · 创启未来

## 技术栈

**前端**：Nuxt 3 + Vue 3 + TypeScript + Tailwind CSS + Pinia + Three.js + GSAP  
**后端**：Rust + axum 0.7 + SQLx + MySQL 8.0 + Redis  
**包管理器**：pnpm（前端必须使用 pnpm，不要用 npm/yarn）

## 项目特点

- **无需登录**：游客直接浏览、点赞、评论
- **智能防刷**：Redis 限流 + 指纹识别 + 敏感词过滤
- **高性能**：Rust 后端 + Nuxt 3 SSR + 游标分页
- **安全可靠**：XSS 防护 + SQL 注入防护 + 数据哈希存储
- **响应式设计**：完美适配 PC、平板、手机

## 快速启动

### 前置条件
- **本地 MySQL**：如果已安装本地 MySQL（端口 3306），无需 Docker
- **Docker Desktop**：如果没有本地 MySQL，需要 Docker 来运行 MySQL 和 Redis

### 一键启动（推荐）
```bash
# Windows
start.bat

# Linux/Mac
./start.sh
```

### 手动启动（本地 MySQL）
```bash
# 1. 创建数据库
mysql -uroot -p123456 -e "CREATE DATABASE IF NOT EXISTS graduation_exhibition CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"

# 2. 运行迁移脚本
for file in backend/migrations/*.sql; do
  mysql -uroot -p123456 --default-character-set=utf8mb4 graduation_exhibition < "$file"
done

# 3. 导入作品数据
mysql -uroot -p123456 --default-character-set=utf8mb4 graduation_exhibition < backend/backend/import_works.sql

# 4. 启动 Redis（如果使用 Docker）
docker run -d -p 6379:6379 redis:7-alpine

# 5. 启动后端（新终端）
cd backend
cargo run

# 6. 启动前端（新终端）
cd frontend
pnpm install  # 首次运行
pnpm dev
```

访问 http://localhost:3000

## 常用命令

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

### 数据库操作
```bash
# 使用本地 MySQL 导入作品数据
mysql -uroot -p123456 --default-character-set=utf8mb4 graduation_exhibition < backend/backend/import_works.sql

# 进入 MySQL 命令行
mysql -uroot -p123456 graduation_exhibition

# 查看表结构
mysql -uroot -p123456 graduation_exhibition -e "SHOW TABLES"

# 查看作品数量
mysql -uroot -p123456 graduation_exhibition -e "SELECT COUNT(*) FROM works"

# 使用 Docker MySQL（如果使用 docker-compose）
docker exec -i media-tech-mysql mysql -uroot -p123456 graduation_exhibition < backend/backend/import_works.sql
```

## 架构概览

### 项目结构
```
media-tech-graduation-exhibition/
├── frontend/                 # Nuxt 3 前端
│   ├── pages/               # 路由页面（4个）
│   ├── components/          # Vue 组件（8个）
│   ├── stores/              # Pinia 状态管理（2个）
│   ├── composables/         # 组合式函数（3个）
│   ├── plugins/             # Nuxt 插件
│   ├── types/               # TypeScript 类型定义
│   └── nuxt.config.ts       # Nuxt 配置
├── backend/                 # Rust axum 后端
│   ├── src/
│   │   ├── main.rs          # 入口文件
│   │   ├── routes/          # API 路由（7个模块）
│   │   ├── models/          # 数据模型（5个）
│   │   ├── services/        # 业务逻辑（4个）
│   │   └── utils/           # 工具函数（2个）
│   ├── migrations/          # 数据库迁移（8个SQL文件）
│   └── backend/import_works.sql  # 作品数据导入脚本
├── docker-compose.yml       # Docker 编排
├── start.sh / start.bat     # 一键启动脚本
└── DEPLOYMENT.md            # 详细部署指南
```

### 核心模块

**前端页面**（基于文件系统路由）：
- `/` - 首页（3D 科技立方体、专业入口）
- `/major/:majorCode` - 专业作品列表（software/electronic/broadcast）
- `/works/:workId` - 作品详情（视频播放、点赞、评论）
- `/ranking` - 点赞热榜（今日/本周/总榜）

**后端 API**：
- `GET /api/health` - 健康检查
- `GET /api/works` - 作品列表（支持分页和专业筛选）
- `GET /api/works/:id` - 作品详情
- `GET /api/interactions/summary` - 批量获取交互统计（含 liked_by_me）
- `GET /api/works/:workId/interaction` - 单个作品交互状态（含 liked_by_me）
- `POST /api/works/:workId/like/toggle` - 切换点赞状态（每日重置）
- `GET /api/works/:workId/comments` - 评论列表（游标分页）
- `POST /api/works/:workId/comments` - 提交评论
- `GET /api/rankings/likes?range=all` - 点赞热榜

### 数据库设计

**works 表**：作品完整信息（id, title, major_code, tags, authors, teacher, poster_url, video_url, introduction）  
**likes 表**：点赞记录（软删除，is_active 字段）  
**comments 表**：评论记录（游标分页，status 字段）  
**abuse_logs 表**：风控日志

## 核心功能实现

### 游客身份识别
- 前端：localStorage 存储 visitor_id（UUID），不同浏览器/用户自动隔离
- 后端：SHA256(work_id + visitor_id + ip + 日期 + server_salt) 生成指纹哈希
- 日期参与哈希，确保每天凌晨后同一用户可重新点赞（日重置机制）
- 校园网多用户场景：visitor_id 为主区分因子，共享公网 IP 不影响独立性
- 自动生成中文昵称：形容词+名词+4位数字（如"快乐熊猫4821"）

### 限流策略（Redis 滑动窗口）
- 点赞：10秒/次（同一指纹对同一作品）
- 评论：30秒/次（同一指纹）
- IP：60秒/20次（同一IP）

### 安全防护
- XSS 防护：HTML 转义（services/sanitize.rs）
- SQL 注入防护：SQLx 参数化查询
- 蜜罐字段：website 字段检测机器人
- 敏感词过滤：本地敏感词库
- IP 和指纹：SHA256 哈希后存储

### 点赞系统
- 原子化切换：`INSERT ... ON DUPLICATE KEY UPDATE is_active = NOT is_active`，消除并发竞争
- 日重置：指纹含日期，每天凌晨后同一用户可对同一作品再次点赞
- 点赞数为跨天累计（所有 is_active=TRUE 记录的总和）
- 取消点赞仅影响当天记录，历史日点赞保留
- 数据库错误会记录日志并返回 500，不再静默吞掉

### 性能优化
- 游标分页：避免深分页（comments 表）
- GPU 加速动画：优先使用 transform 和 opacity
- Nuxt 3 SSR：首屏加载快、SEO 友好

## 代码风格

### 前端（Vue 3 + TypeScript）
- 使用 Composition API + `<script setup>` 语法
- 状态管理：Pinia stores（visitor, interactions）
- 样式：Tailwind CSS 工具类，避免自定义 CSS
- 动画：GSAP 3.12+，优先使用 GPU 加速属性

### 后端（Rust + axum）
- 模块化路由：routes/ 目录，每个功能独立文件
- 数据模型：models/ 目录，使用 SQLx FromRow
- 业务逻辑：services/ 目录（fingerprint, rate_limit, sanitize）
- 错误处理：thiserror 定义自定义错误类型
- 数据库查询：必须使用参数化查询

## 环境配置

### 前端环境变量（frontend/.env）
```env
NUXT_PUBLIC_API_BASE=http://localhost:8080/api
NUXT_PUBLIC_COS_BASE_URL=https://whcm-1353140174.cos.ap-nanjing.myqcloud.com
```

### 后端环境变量（backend/.env）
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

**重要**：
- 数据库密码默认为 `123456`（本地开发）
- 数据库名必须是 `graduation_exhibition`（不是 `demos`）
- 生产环境必须修改 `SERVER_SALT`

## 项目完成状态

### 已完成（100%）
- ✅ 前端 4 个页面 + 8 个组件
- ✅ 后端完整 API（10+ 个接口）
- ✅ 数据库表结构（4 个表）
- ✅ 游客指纹识别（SHA256）
- ✅ Redis 限流（滑动窗口）
- ✅ 内容审核（敏感词、HTML 转义）
- ✅ 点赞系统（日重置 + 原子化切换）
- ✅ 评论系统（游标分页）
- ✅ 排行榜（全部/今日/本周）
- ✅ 33 个作品完整数据（含标题、作者、简介等）
- ✅ 33 个作品视频文件（腾讯云 COS 存储）
- ✅ 一键启动脚本

### 待完成（首次部署）
- ⚠️ 启动 MySQL 和 Redis（Docker 或本地）
- ⚠️ 运行数据库迁移脚本
- ⚠️ 导入作品数据
- ⚠️ 启动后端和前端服务
- ⚠️ 完整功能测试

### 可选优化
- 🔲 DeepSeek API 评论审核集成
- 🔲 作品管理后台（/admin/works）
- 🔲 CDN 加速静态资源

## 重要注意事项

### 包管理器
**前端必须使用 pnpm**，不要使用 npm 或 yarn。项目已配置 `"packageManager": "pnpm@9.0.0"`。

### 数据库
使用 MySQL 8.0。数据库名：`graduation_exhibition`。

**重要**：
- docker-compose.yml 中的 `MYSQL_DATABASE` 应该是 `graduation_exhibition`
- 导入数据时必须使用 `--default-character-set=utf8mb4`，否则中文会乱码
- 数据库密码默认为 `123456`（本地开发）

### 静态文件
- **视频和海报**：存储在腾讯云 COS 对象存储
  - COS 基础 URL：`https://whcm-1353140174.cos.ap-nanjing.myqcloud.com`
  - 软件工程：`/resource/SE/001.mp4` - `/resource/SE/012.mp4`（海报为 `.jpg`）
  - 电子信息：`/resource/Ele/001.mp4` - `/resource/Ele/012.mp4`（海报为 `.jpg`）
  - 广播电视：`/resource/RTE/001.mp4` - `/resource/RTE/012.mp4`（海报为 `.jpg`）
- 前端直接访问 COS URL，无需通过后端代理
- 数据库中存储完整的 COS URL

### 前后端数据格式
- **后端 API 返回**：下划线格式（`major_code`, `poster_url`, `like_count`）
- **前端类型定义**：`frontend/types/work.ts` 使用下划线格式匹配 API
- **Pinia Store**：内部转换为驼峰格式（`majorCode`, `posterUrl`, `likeCount`）

### 端口占用
- 3000：前端
- 8080：后端
- 3306：MySQL
- 6379：Redis

## 参考文档

- [README.md](README.md) - 项目说明
- [PRD.md](PRD.md) - 产品需求文档
- [DEPLOYMENT.md](DEPLOYMENT.md) - 详细部署指南
- [COS_MIGRATION.md](COS_MIGRATION.md) - COS 资源迁移说明
- [毕业设计作品汇总.md](毕业设计作品汇总.md) - 作品信息源文件

## 设计规范

### 视觉风格
- 浅色科技感、玻璃拟态（glassmorphism）
- 蓝白配色：#1466ff（主蓝）、#37c8ff（青色）
- 3D 科技立方体、粒子光点、数据网格背景

### 动画实现（GSAP 3.12+）
```typescript
import { gsap } from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'

gsap.registerPlugin(ScrollTrigger)

// 淡入动画
gsap.from('.card', {
  opacity: 0,
  y: 50,
  duration: 0.8,
  stagger: 0.2
})

// 滚动触发
gsap.to('.element', {
  scrollTrigger: {
    trigger: '.element',
    start: 'top center',
    scrub: true
  },
  x: 100
})
```

**性能优化**：
- 优先使用 `transform` 和 `opacity`（GPU 加速）
- 避免动画 `width`、`height`、`top`、`left`（触发重排）
- 使用 `will-change` 提示浏览器优化

## 故障排查

### 前端无法启动
```bash
# 清理依赖重新安装
cd frontend
rm -rf node_modules pnpm-lock.yaml
pnpm install
```

### 后端编译失败
```bash
# 清理并重新编译
cd backend
cargo clean
cargo build
```

### 数据库连接失败
```bash
# 检查 MySQL 容器状态
docker ps | grep mysql

# 查看 MySQL 日志
docker logs media-tech-mysql

# 重启 MySQL
docker restart media-tech-mysql
```

### API 调用失败
- 检查后端是否启动：http://localhost:8080/api/health
- 检查前端环境变量：`frontend/.env` 中的 `NUXT_PUBLIC_API_BASE`
- 查看浏览器控制台错误信息

## 开发建议

1. **修改代码前**：先阅读 CLAUDE.md 了解项目架构
2. **添加新功能**：参考现有模块的实现模式（如 likes.rs、comments.rs）
3. **修改数据库**：在 migrations/ 目录添加新的迁移文件
4. **部署前**：查看 [DEPLOYMENT.md](DEPLOYMENT.md) 的部署检查清单
