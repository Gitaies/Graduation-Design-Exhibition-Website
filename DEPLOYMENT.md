# 部署指南

## 目录

- [本地部署（开发测试）](#本地部署开发测试)
- [生产部署（SSG + Nginx）](#生产部署ssg--nginx)
- [公网测试（ngrok）](#公网测试ngrok)
- [数据库管理](#数据库管理)
- [常见问题](#常见问题)

---

## 本地部署（开发测试）

```bash
# 1. 启动 MySQL + Redis
docker-compose up -d

# 2. 初始化数据库
mysql -uroot -p123456 -e "CREATE DATABASE IF NOT EXISTS graduation_exhibition CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"
for f in backend/migrations/*.sql; do mysql -uroot -p123456 --default-character-set=utf8mb4 graduation_exhibition < "$f"; done
mysql -uroot -p123456 --default-character-set=utf8mb4 graduation_exhibition < backend/backend/import_works.sql

# 3. 启动后端 (http://localhost:8080)
cd backend && cargo run

# 4. 启动前端开发模式 (http://localhost:3000)
cd frontend && pnpm install && pnpm dev
```

---

## 生产部署（SSG + Nginx）

### 构建静态文件

确保后端正在运行（预渲染需要获取作品数据）：

```bash
cd frontend
rm -rf .nuxt .output
pnpm generate
```

产出在 `.output/public/`，包含预渲染好的所有 HTML + JS + CSS。

### Nginx 配置

将 [nginx/graduation.conf](nginx/graduation.conf) 复制到服务器 `/etc/nginx/sites-available/`，修改：

1. `server_name` — 替换为你的域名
2. `$nuxt_root` — 指向 `.output/public/` 的实际路径
3. 配置 SSL 证书（Let's Encrypt）

```bash
sudo cp nginx/graduation.conf /etc/nginx/sites-available/graduation
sudo ln -s /etc/nginx/sites-available/graduation /etc/nginx/sites-enabled/
sudo rm /etc/nginx/sites-enabled/default
sudo nginx -t && sudo nginx -s reload
```

### SSL 证书

```bash
sudo apt install certbot python3-certbot-nginx -y
sudo certbot --nginx -d yourdomain.com
sudo certbot renew --dry-run  # 测试自动续期
```

### 完整部署架构

```
浏览器 → Nginx (:80/443)
            ├── /api/*    → Rust 后端 :8080
            └── /*        → .output/public/ 静态文件
```

- API 地址配置为 `/api`（相对路径），前端 `.env`：`NUXT_PUBLIC_API_BASE=/api`
- 后端需配置 `SERVER_SALT` 和 `CORS_ALLOWED_ORIGINS` 为实际域名

---

## 公网测试（ngrok）

买服务器前临时公网演示：

```bash
# 安装 ngrok 并配置
ngrok config add-authtoken <你的token>
ngrok http 80
```

将生成的 `https://xxx.ngrok-free.app` 分享给他人在线访问。免费版每次重启地址会变，有流量限制。

---

## 数据库管理

### 备份

```bash
# 本地 MySQL
mysqldump -uroot -p123456 --default-character-set=utf8mb4 graduation_exhibition > backup_$(date +%Y%m%d).sql

# Docker MySQL
docker exec media-tech-mysql mysqldump -uroot -p123456 graduation_exhibition > backup_$(date +%Y%m%d).sql
```

### 恢复

```bash
mysql -uroot -p123456 --default-character-set=utf8mb4 graduation_exhibition < backup_20260512.sql
```

### 查看数据

```bash
mysql -uroot -p123456 graduation_exhibition -e "SELECT COUNT(*) FROM works"
mysql -uroot -p123456 graduation_exhibition -e "SELECT COUNT(*) FROM likes WHERE is_active = 1"
mysql -uroot -p123456 graduation_exhibition -e "SELECT COUNT(*) FROM comments WHERE status = 'visible'"
```

### 使用 Docker MySQL

```bash
docker exec -i media-tech-mysql mysql -uroot -p123456 --default-character-set=utf8mb4 graduation_exhibition < backend/backend/import_works.sql
docker exec -it media-tech-mysql mysql -uroot -p123456 graduation_exhibition
```

---

## 常见问题

### 外部设备访问 API 失败

`frontend/.env` 中 `NUXT_PUBLIC_API_BASE` 必须设为 `/api`（相对路径），不能写 `http://localhost:8080/api`。其他设备访问时 localhost 指向的是设备自身而非服务器。

### 作品详情页无数据

SSG 构建时后端必须运行，否则作品页无法获取数据。清理缓存重建：

```bash
cd frontend && rm -rf .nuxt .output && pnpm generate
```

### Docker 启动失败

```bash
docker-compose down
docker-compose up -d
docker logs media-tech-mysql
```

### 前端依赖问题

```bash
cd frontend
rm -rf node_modules pnpm-lock.yaml .nuxt .output
pnpm install
```

### Nginx 502 Bad Gateway

检查后端是否运行：`curl http://localhost:8080/api/health`

### 图片/视频无法加载

检查 COS 存储桶 CORS 配置是否允许你的域名。

---

## 环境要求

| 组件 | 版本 |
|------|------|
| Node.js | >= 18 |
| pnpm | >= 9 |
| Rust | >= 1.70 |
| MySQL | 8.0 |
| Redis | 7 |
| Nginx | 1.24+ |

## COS 资源

作品视频和封面存储在腾讯云 COS：
- 基础 URL：`https://whcm-1353140174.cos.ap-nanjing.myqcloud.com`
- 路径格式：`/resource/{SE|Ele|RTE}/{001-012}.mp4`
- 详见 [COS_MIGRATION.md](COS_MIGRATION.md)
