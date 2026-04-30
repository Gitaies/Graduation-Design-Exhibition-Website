# 部署指南

## 📋 目录

- [快速启动](#快速启动)
- [手动启动](#手动启动)
- [测试功能](#测试功能)
- [常见问题](#常见问题)
- [管理命令](#管理命令)
- [生产环境部署](#生产环境部署)
- [性能优化](#性能优化)
- [监控和日志](#监控和日志)
- [备份和恢复](#备份和恢复)

---

## 快速启动

### Windows 用户
双击运行 `start.bat`，或在命令行中执行：
```cmd
start.bat
```

### Linux/Mac 用户
```bash
chmod +x start.sh
./start.sh
```

脚本会自动完成：
1. ✅ 启动 MySQL 和 Redis
2. ✅ 等待数据库就绪
3. ✅ 导入作品数据
4. ✅ 检查环境配置

启动完成后：
- **前端**：http://localhost:3000
- **后端 API**：http://localhost:8080
- **健康检查**：http://localhost:8080/api/health

---

## 手动启动

### 1. 启动数据库和 Redis

```bash
docker-compose up -d
```

### 2. 等待 MySQL 启动并导入数据

```bash
# 等待 15 秒
sleep 15

# 导入作品数据
docker exec -i media-tech-mysql mysql -uroot -ppassword graduation_exhibition < backend/import_works.sql
```

### 3. 启动后端

```bash
cd backend
cargo run
```

后端将在 http://localhost:8080 启动

### 4. 启动前端

```bash
cd frontend

# 首次运行需要安装依赖（必须使用 pnpm）
pnpm install

# 启动开发服务器
pnpm dev
```

前端将在 http://localhost:3000 启动

### 5. 访问网站

打开浏览器访问：http://localhost:3000

---

## 测试功能

### 访问页面

- **首页**: http://localhost:3000
- **专业展区**: http://localhost:3000/majors
- **软件工程作品**: http://localhost:3000/major/software
- **电子信息工程作品**: http://localhost:3000/major/electronic
- **广播电视工程作品**: http://localhost:3000/major/broadcast
- **作品详情**: http://localhost:3000/works/w001
- **点赞热榜**: http://localhost:3000/ranking
- **关于毕展**: http://localhost:3000/about

### 测试点赞功能
1. 进入任意作品详情页
2. 点击"点赞"按钮
3. 观察点赞数变化
4. 再次点击取消点赞

### 测试评论功能
1. 进入任意作品详情页
2. 在评论框输入内容
3. 点击"发送"
4. 观察评论列表更新

### 测试限流功能
1. 快速连续点击点赞按钮
2. 应该看到"操作太频繁"提示
3. 等待 10 秒后可再次点赞

---

## 常见问题

### 1. Docker 启动失败

**问题**: `docker-compose up -d` 失败

**解决方案**:
- 确保 Docker Desktop 已启动
- 检查端口 3306 和 6379 是否被占用
- 运行 `docker-compose down` 清理后重试

### 2. MySQL 连接失败

**问题**: 后端无法连接 MySQL

**解决方案**:
- 检查 `backend/.env` 中的 `DATABASE_URL`
- 确保 MySQL 容器正在运行：`docker ps`
- 查看 MySQL 日志：`docker logs media-tech-mysql`

### 3. 前端 API 调用失败

**问题**: 前端显示"获取数据失败"

**解决方案**:
- 确保后端已启动（http://localhost:8080）
- 检查 `frontend/.env` 中的 `NUXT_PUBLIC_API_BASE`
- 打开浏览器控制台查看错误信息

### 4. 作品数据为空

**问题**: 作品列表为空

**解决方案**:
```bash
# 重新导入数据
docker exec -i media-tech-mysql mysql -uroot -ppassword graduation_exhibition < backend/import_works.sql
```

### 5. Rust 编译错误

**问题**: Rust 编译失败

**解决方案**:
- 确保 Rust 版本 >= 1.70：`rustc --version`
- 清理并重新编译：
  ```bash
  cd backend
  cargo clean
  cargo build
  ```

### 6. 前端依赖安装失败

**问题**: `pnpm install` 失败

**解决方案**:
- 确保使用 pnpm（不是 npm）：`pnpm --version`
- 清理缓存：
  ```bash
  cd frontend
  rm -rf node_modules
  rm pnpm-lock.yaml
  pnpm install
  ```

---

## 管理命令

### 查看日志

```bash
# 查看所有容器日志
docker-compose logs -f

# 查看 MySQL 日志
docker logs -f media-tech-mysql

# 查看 Redis 日志
docker logs -f media-tech-redis
```

### 停止服务

```bash
# 停止所有容器
docker-compose down

# 停止并删除数据卷（会清空数据库）
docker-compose down -v
```

### 重启服务

```bash
# 重启所有容器
docker-compose restart

# 重启 MySQL
docker restart media-tech-mysql
```

### 进入容器

```bash
# 进入 MySQL 容器
docker exec -it media-tech-mysql mysql -uroot -ppassword graduation_exhibition

# 进入 Redis 容器
docker exec -it media-tech-redis redis-cli
```

### 查看数据库

```bash
# 查看所有表
docker exec media-tech-mysql mysql -uroot -ppassword graduation_exhibition -e "SHOW TABLES"

# 查看作品数量
docker exec media-tech-mysql mysql -uroot -ppassword graduation_exhibition -e "SELECT COUNT(*) FROM works"

# 查看点赞数量
docker exec media-tech-mysql mysql -uroot -ppassword graduation_exhibition -e "SELECT COUNT(*) FROM likes WHERE is_active = 1"
```

---

## 生产环境部署

### 1. 修改配置

#### 后端配置 (`backend/.env`)
```env
APP_HOST=0.0.0.0
APP_PORT=8080
DATABASE_URL=mysql://root:YOUR_STRONG_PASSWORD@localhost:3306/graduation_exhibition
REDIS_URL=redis://localhost:6379
SERVER_SALT=YOUR_RANDOM_SALT_HERE  # 必须修改！
CORS_ALLOWED_ORIGINS=https://yourdomain.com
TRUST_PROXY=true
```

#### 前端配置 (`frontend/.env`)
```env
NUXT_PUBLIC_API_BASE=https://api.yourdomain.com/api
```

### 2. 构建生产版本

#### 后端
```bash
cd backend
cargo build --release
./target/release/media-tech-exhibition-backend
```

#### 前端
```bash
cd frontend
pnpm build
pnpm preview  # 预览
```

### 3. 使用 Nginx 反向代理

```nginx
# 前端
server {
    listen 80;
    server_name yourdomain.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}

# 后端 API
server {
    listen 80;
    server_name api.yourdomain.com;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```

### 4. 配置 SSL

使用 Let's Encrypt 免费证书：
```bash
sudo certbot --nginx -d yourdomain.com -d api.yourdomain.com
```

### 5. 使用 systemd 管理服务

#### 后端服务 (`/etc/systemd/system/graduation-backend.service`)
```ini
[Unit]
Description=Graduation Exhibition Backend
After=network.target mysql.service redis.service

[Service]
Type=simple
User=www-data
WorkingDirectory=/path/to/backend
ExecStart=/path/to/backend/target/release/media-tech-exhibition-backend
Restart=always

[Install]
WantedBy=multi-user.target
```

#### 前端服务 (`/etc/systemd/system/graduation-frontend.service`)
```ini
[Unit]
Description=Graduation Exhibition Frontend
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/path/to/frontend
ExecStart=/usr/bin/pnpm start
Restart=always

[Install]
WantedBy=multi-user.target
```

启动服务：
```bash
sudo systemctl enable graduation-backend
sudo systemctl enable graduation-frontend
sudo systemctl start graduation-backend
sudo systemctl start graduation-frontend
```

---

## 性能优化

### 1. CDN 加速
- 将 `backend/static/` 目录上传到 CDN
- 修改 `poster_url` 和 `video_url` 为 CDN 地址

### 2. 数据库优化
- 定期清理过期数据
- 添加必要的索引
- 配置 MySQL 缓存

### 3. Redis 优化
- 配置持久化
- 设置合理的过期时间
- 监控内存使用

### 4. 前端优化
- 启用 Nuxt 3 的静态生成（SSG）
- 配置图片懒加载
- 启用 Gzip 压缩

---

## 监控和日志

### 应用日志

后端日志会输出到控制台，建议使用日志收集工具：
- **开发环境**: 直接查看控制台
- **生产环境**: 使用 systemd 或 PM2 管理进程并收集日志

### 数据库监控

```bash
# 查看 MySQL 状态
docker exec media-tech-mysql mysql -uroot -ppassword -e "SHOW STATUS"

# 查看慢查询
docker exec media-tech-mysql mysql -uroot -ppassword -e "SHOW VARIABLES LIKE 'slow_query%'"
```

### Redis 监控

```bash
# 查看 Redis 信息
docker exec media-tech-redis redis-cli INFO

# 监控实时命令
docker exec media-tech-redis redis-cli MONITOR
```

---

## 备份和恢复

### 数据库备份

```bash
# 备份
docker exec media-tech-mysql mysqldump -uroot -ppassword graduation_exhibition > backup_$(date +%Y%m%d).sql

# 恢复
docker exec -i media-tech-mysql mysql -uroot -ppassword graduation_exhibition < backup_20260427.sql
```

### Redis 备份

```bash
# 备份
docker exec media-tech-redis redis-cli SAVE
docker cp media-tech-redis:/data/dump.rdb ./redis_backup_$(date +%Y%m%d).rdb

# 恢复
docker cp redis_backup_20260427.rdb media-tech-redis:/data/dump.rdb
docker restart media-tech-redis
```

---

## 环境要求

### 开发环境
- Docker Desktop（必须）
- Node.js 18+（前端开发）
- Rust 1.70+（后端开发）
- pnpm 9.0+（前端包管理器）

### 端口占用
- 3000：前端
- 8080：后端
- 3306：MySQL
- 6379：Redis

---

## 技术支持

如遇到问题，请查看：
1. [PROJECT_STATUS.md](PROJECT_STATUS.md) - 项目状态和测试清单
2. [CLAUDE.md](CLAUDE.md) - 完整项目文档
3. [README.md](README.md) - 项目说明

或提交 Issue 到项目仓库。
