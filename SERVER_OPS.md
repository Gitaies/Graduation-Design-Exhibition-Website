# 云服务器部署与运维文档

**服务器**：腾讯云 CVM 2核2G 3Mbps | 公网 IP `122.51.232.24` | Ubuntu 22.04
**部署日期**：2026-05-13 | **最后更新**：2026-05-14
**部署状态**：正常运行

---

## 架构概览

```
浏览器 (公网)
    │
    ▼
┌──────────────────────────────────────────────────┐
│  Nginx :80                                       │
│  ├── /                    静态 HTML (SSG)        │
│  ├── /_nuxt/              静态 JS/CSS/图片       │
│  ├── /data/               作品封面 (本地 WebP)    │
│  ├── /images/             静态图片 (WebP)        │
│  ├── /api/*          →    后端 :8080             │
│  └── /static/*       →    后端 :8080             │
└──────────────────────────────────────────────────┘
    │                         │
    │ 静态文件来自:              │ 作品视频来自:
    │ .output/public/           │ COS (whcm-1353140174)
    ▼                           ▼
┌──────────────────┐    ┌──────────────────────────┐
│  Rust axum :8080  │    │  腾讯云 COS 对象存储      │
│  ├── /api/health  │    │  ├── Se/*.mp4 (视频)     │
│  ├── /api/works/* │    │  ├── Ele/*.mp4 (视频)    │
│  ├── /api/rank... │    │  └── Rte/*.mp4 (视频)    │
│  └── /api/inter... │    │  Cache-Control: 7天视频   │
└────┬──────────────┘    │  Referer 防盗链: 已开启 │
     │                    └──────────────────────────┘
     ▼
┌──────────┐  ┌──────┐
│ MySQL 8  │  │Redis7│
│ (Docker) │  │(Docker)│
└──────────┘  └──────┘
```

---

## 服务器配置清单

### 已部署服务

| 服务 | 管理方式 | 开机自启 | 配置文件 |
|------|---------|---------|---------|
| Nginx | systemd | 是 | `/etc/nginx/sites-enabled/default` |
| Backend | systemd (`graduation-backend`) | 是 | `/etc/systemd/system/graduation-backend.service` |
| MySQL | Docker Compose | 是 | `docker-compose.yml` |
| Redis | Docker Compose | 是 | `docker-compose.yml` |

### 关键路径

```
/home/ubuntu/Graduation-Design-Exhibition-Website/
├── frontend/
│   ├── .output/public/          ← Nginx 从这里读静态文件
│   │   └── data/                ← 作品封面 WebP（w001-w033.webp）
│   ├── server/api/[...].ts      ← SSG 构建时的 API 代理（关键！不能删）
│   ├── nuxt.config.ts
│   └── .env
├── backend/
│   ├── target/release/media-tech-exhibition-backend  ← 后端二进制
│   ├── .env
│   └── migrations/
├── docker-compose.yml
├── update.sh                    ← 快速更新脚本
└── SERVER_OPS.md
```

---

## 部署时遇到的问题与解决

### 问题 1：SSG 构建无法访问后端 API

**现象**：`pnpm generate` 时 `$fetch('/api/works/w001')` 返回 404，作品页无数据。

**根因**：Nuxt 3.21 的 `routeRules.proxy` 在 SSG 预渲染阶段不生效，只对 Nitro 生产服务器（SSR）有效。

**解决**：创建 `frontend/server/api/[...].ts` 作为 Nitro 服务端 catch-all 路由，SSG 期间拦截所有 `/api/**` 请求并转发到 `localhost:8080`。

### 问题 2：Nginx 无法访问静态文件

**现象**：Nginx 返回 404，错误日志 `(13: Permission denied)`。

**根因**：`/home/ubuntu` 目录权限 750，Nginx（www-data 用户）无法遍历。

**解决**：`chmod o+x /home/ubuntu` + `chmod -R o+rX .../public`

### 问题 3：COS 流量异常扣费 30 元/天

**现象**：COS 一天扣费 30.64 元，之前每天 1-2 元。

**根因**：数据库 33 个作品的 `poster_url = video_url = 同一个 .mp4`，WorkCard / HomeRanking / ranking 三个组件用 `<video>` 标签加载 MP4 做缩略图。每张卡片下载 5-30MB 视频文件，全站浏览一遍产生 ~400MB 视频流量。

**解决**：
1. 从视频截取第 5 秒帧生成封面 JPG（每张 50-450KB）
2. 转为 WebP 格式存入本地 `frontend/public/data/w001.webp ~ w033.webp`（总计 ~4.5MB）
3. 数据库 `poster_url` 改用本地路径 `/data/wXXX.webp`，`video_url` 保持 COS MP4
4. 卡片组件改用 `<img>` 加载 WebP 封面（不再触发视频下载）
5. 详情页 `<video>` 加 `poster` 属性
6. COS 设置 Cache-Control（视频 7 天）
7. COS 开启 Referer 防盗链

**效果**：封面图片无 COS 流量，全站浏览一趟的 COS 流量从 ~400MB 降到 ~15MB。

### 问题 4：首页首屏加载慢

**现象**：公网访问首页，背景图、粒子动画图标加载很久。

**根因**：首页 5 张 PNG 图片共 5.4MB（hero_bg.png 2.78MB + 3 个 icon 2.1MB + hero_icon.png 487KB），3Mbps 带宽下仅图片就要下载 15 秒。

**解决**：全部 PNG/JPG 转 WebP（Pillow quality 85），5.4MB → 334KB（-94%），3Mbps 下图片下载 < 1 秒。

---

## 性能优化记录

### 图片格式优化

| 文件 | 原格式 | 原大小 | WebP | 缩减 |
|------|--------|--------|------|------|
| hero_bg (首页背景) | PNG | 2,716 KB | 52 KB | -98% |
| hero_icon (装饰图标) | PNG | 475 KB | 56 KB | -88% |
| icon_1 (软件工程) | PNG | 750 KB | 61 KB | -92% |
| icon_2 (电子信息) | PNG | 739 KB | 61 KB | -92% |
| icon_3 (广播电视) | PNG | 561 KB | 55 KB | -90% |
| ftbg1 (Footer 背景) | PNG | 70 KB | 21 KB | -70% |
| bga (关于页面背景) | JPG | 52 KB | 11 KB | -79% |
| logo | PNG | 26 KB | 17 KB | -36% |
| **合计** | | **5,391 KB** | **334 KB** | **-94%** |

### COS 流量优化

| 措施 | 效果 |
|------|------|
| poster_url .mp4 → 本地 .webp | 卡片封面零 COS 流量 |
| `<video>` → `<img>` 加载卡片封面 | 不再触发视频预加载 |
| `<video poster>` 属性 | 详情页播放前不下载视频 |
| Cache-Control: 7天(视频) | 回头客不产生 COS 请求 |
| Referer 防盗链 | 外部盗链被拦截 |

---

## 日常运维

### 查看服务状态

```bash
systemctl status nginx graduation-backend
sudo docker ps
sudo journalctl -u graduation-backend -f       # 后端日志
sudo tail -f /var/log/nginx/graduation_access.log  # 访问日志
```

### 重启服务

```bash
sudo systemctl restart nginx
sudo systemctl restart graduation-backend
```

### 数据库备份

```bash
sudo docker exec media-tech-mysql mysqldump -uroot -p123456 graduation_exhibition \
  > ~/backup_$(date +%Y%m%d).sql
```

---

## 更新部署流程

### 只改前端

```bash
# 本地
cd frontend
tar --exclude='node_modules' --exclude='.nuxt' --exclude='.output' -czf ../frontend-update.tar.gz .
scp ../frontend-update.tar.gz ubuntu@122.51.232.24:~/

# 服务器
ssh ubuntu@122.51.232.24
cd ~/Graduation-Design-Exhibition-Website/frontend
tar -xzf ~/frontend-update.tar.gz
pnpm install
rm -rf .nuxt .output && pnpm generate
```

### 只改后端

```bash
# 本地
cd backend
tar --exclude='target' -czf ../backend-update.tar.gz .
scp ../backend-update.tar.gz ubuntu@122.51.232.24:~/

# 服务器
ssh ubuntu@122.51.232.24
cd ~/Graduation-Design-Exhibition-Website/backend
tar -xzf ~/backend-update.tar.gz
cargo build --release
sudo systemctl restart graduation-backend
```

### 一键更新（服务器已有 update.sh）

上传 tar 包后 SSH 执行：`~/Graduation-Design-Exhibition-Website/update.sh`

---

## 故障排查速查表

| 症状 | 可能原因 | 排查命令 |
|------|---------|---------|
| 网站打不开 | Nginx 挂了 | `sudo systemctl status nginx` |
| API 返回 502 | 后端挂了 | `sudo systemctl status graduation-backend` |
| 数据不更新 | SSG 没重构建 | `ls -la frontend/.output/public/index.html` 看修改时间 |
| 图片不显示 | 封面文件缺失或 COS 防盗链 | 检查 `frontend/public/data/` 下 WebP 文件；视频图片检查 COS Referer 白名单 |
| COS 费用异常 | 缓存头丢失 | `curl -sI <cos-url> \| grep cache-control` |

---

## 本地项目关键文件

| 文件 | 用途 | 注意 |
|------|------|------|
| `frontend/server/api/[...].ts` | SSG API 代理 | **不能删除** |
| `frontend/.env` | `NUXT_PUBLIC_API_BASE=/api` | 相对路径 |
| `frontend/nuxt.config.ts` | 无 routeRules proxy | 靠 server/api 代理 |
| `frontend/composables/useCosUrl.ts` | **已删除** | COS 截帧 API 不可用 |

### 本地与服务器差异

| 配置 | 本地 | 服务器 |
|------|------|--------|
| `frontend/.env` API_BASE | `/api` (本地 dev) | `/api` (通过 Nginx) |
| `backend/.env` 数据库密码 | 本地密码 | 服务器密码 |
| Docker | Docker Desktop | Docker Engine |
| 封面图片 | 本地 WebP (`public/data/`) | WebP (SSG 输出到 `.output/public/data/`) |
| 视频资源 | COS (`whcm-1353140174`) | COS（同本地） |

---

## 安全提醒

- 当前 HTTP 访问，无 SSL 证书
- 数据库密码弱，建议修改
- `SERVER_SALT` 建议改为随机字符串
- 腾讯云安全组只开放 80 / 22 端口
- COS 已开启 Referer 防盗链

---

*文档生成日期：2026-05-13 | 最后更新：2026-05-14*
