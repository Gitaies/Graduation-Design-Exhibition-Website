# PRD｜传媒技术学院 2026 届毕业设计展网站

> **更新说明（2026-04-26）**：  
> 本 PRD 是原始需求文档，实际实施中技术栈有所调整：  
> - 前端：Nuxt 3
> - 数据库：MySQL 8.0
> - 其他技术栈保持不变：Vue 3、TypeScript、Rust axum、Redis
> 详细架构调整见 [findings.md](findings.md) 和 [ARCHITECTURE_REFACTOR.md](ARCHITECTURE_REFACTOR.md)

> 用途：本 PRD 用于交给 Claude Code 或其他 AI 编程工具，一次性生成完整项目框架。  
> 技术栈：前端使用 Vben Admin / Vue 3 / TypeScript；后端使用 Rust axum；数据库建议 PostgreSQL，缓存/限流建议 Redis。  
> 项目性质：公开展示型毕业设计展网站，游客无需登录即可浏览、点赞、评论和查看热榜。

---

## 1. 项目概述

### 1.1 项目名称

传媒技术学院 2026 届毕业设计展网站

### 1.2 展览主题

**智媒融合 · 创启未来**

### 1.3 核心定位

本项目是一个面向公众开放的毕业设计展览网站，用于展示传媒技术学院 2026 届毕业设计作品。网站整体采用**浅色科技感、玻璃拟态、蓝白视觉、立体科技元素、数据网格背景**的设计风格。

网站需覆盖三个专业方向：

| 专业 | 专业主题 | 副标题 |
|---|---|---|
| 软件工程 | 数智焕新 | 数智驱动 · 焕新应用 |
| 电子信息工程 | 芯火智造 | 芯火相传 · 智造终端 |
| 广播电视工程 | 虚实共生 | 虚实共生 · 视界新生 |

### 1.4 项目目标

1. 打造一个视觉统一、科技感强、适合毕业设计展的公开网站。
2. 游客无需登录即可自由浏览作品、播放视频、点赞、评论、查看热榜。
3. 前端页面结构清晰，方便使用 Vben Admin 快速落地。
4. 后端只实现轻量交互能力：点赞、评论、热榜、反刷限制。
5. 支持 PC、平板、手机响应式展示。
6. 考虑恶意刷点赞、刷评论、脚本提交、XSS 注入等基础安全问题。

---

## 2. 使用说明：给 Claude Code 的开发目标

请根据本文档生成一个完整的全栈项目，建议使用 monorepo 结构：

```text
media-tech-graduation-exhibition/
├── frontend/              # Vben Admin / Vue 3 / TypeScript 前端
├── backend/               # Rust axum 后端
├── docs/                  # 项目说明文档
├── docker-compose.yml     # PostgreSQL、Redis、后端服务编排
├── README.md
└── .gitignore
```

开发目标：

1. 生成可运行的前端项目框架。
2. 生成可运行的 Rust axum 后端项目框架。
3. 前端页面先使用本地静态作品数据，点赞、评论、热榜数据通过后端接口获取。
4. 后端不需要实现用户登录系统，不需要后台管理系统。
5. 后端需要实现点赞、取消点赞、评论提交、评论列表、点赞热榜、交互统计接口。
6. 提供数据库 migrations、环境变量示例、API 类型定义、基础错误处理、限流逻辑。
7. 前后端都要有清晰的目录结构、注释和 README 启动说明。

---

## 3. 用户角色

### 3.1 游客

游客是本项目的主要用户，无需注册或登录。

游客可以：

- 浏览首页。
- 查看三大专业展区。
- 查看专业作品列表。
- 查看作品详情。
- 播放作品视频。
- 点赞作品。
- 取消点赞。
- 发表评论。
- 查看作品评论。
- 查看点赞热榜。

## 4. 产品范围

### 4.1 本期必须实现

| 模块 | 是否必须 | 说明 |
|---|---:|---|
| 首页封面页 | 是 | 展示主题、学院信息、三大专业入口 |
| 专业展区页 | 是 | 展示三个专业卡片 |
| 专业作品列表页 | 是 | 按专业展示作品列表 |
| 作品详情页 | 是 | 展示作品视频、简介、点赞、评论 |
| 点赞热榜页 | 是 | 按点赞数排序展示作品 |
| 响应式适配 | 是 | PC、平板、手机 |
| 点赞接口 | 是 | 游客无需登录，限制重复点赞 |
| 评论接口 | 是 | 游客无需登录，自动生成游客昵称和评论 ID |
| 反刷机制 | 是 | 限流、去重、敏感词、XSS 处理 |
| 数据库持久化 | 是 | 评论和点赞数据需要存入数据库 |

## 5. 信息架构与页面路由

### 5.1 前端路由

| 页面 | 路由 | 说明 |
|---|---|---|
| 首页封面页 | `/` | 展览首页，主题视觉入口 |
| 专业展区页 | `/majors` | 三大专业入口 |
| 专业作品列表页 | `/major/:majorCode` | 按专业展示作品 |
| 作品详情页 | `/works/:workId` | 展示单个作品详情、点赞、评论 |
| 点赞热榜页 | `/ranking` | 展示点赞排行榜 |
| 关于毕展页 | `/about` | 展示主办方、联系方式、版权信息 |

### 5.2 majorCode 约定

| majorCode | 专业 |
|---|---|
| `software` | 软件工程 |
| `electronic` | 电子信息工程 |
| `broadcast` | 广播电视工程 |

---

## 6. 视觉设计要求

### 6.1 整体风格

必须贴近用户提供的原型图风格，而不是重新设计成完全不同的后台页面或暗黑科技风。

关键词：

- 浅色科技系。
- 蓝白主色。
- 玻璃拟态卡片。
- 轻量阴影。
- 细线网格。
- 3D 科技立方体。
- 粒子光点。
- 数据波浪背景。
- 大留白。
- UI/UX 展板感。

不要：

- 不要做成深色赛博朋克风。
- 不要做成普通后台管理系统风格。
- 不要大面积使用黑色背景。
- 不要使用过重的紫色、红色或霓虹色。
- 不要让页面过于拥挤。

### 6.2 色彩规范

建议使用 CSS 变量：

```css
:root {
  --primary-blue: #1466ff;
  --primary-blue-dark: #0b4edc;
  --primary-cyan: #37c8ff;
  --text-main: #0f172a;
  --text-secondary: #64748b;
  --text-light: #94a3b8;
  --bg-page: #f5f8ff;
  --bg-card: rgba(255, 255, 255, 0.82);
  --border-light: rgba(30, 100, 220, 0.12);
  --shadow-blue: rgba(64, 132, 255, 0.18);
}
```

### 6.3 字体规范

中文建议：

- 系统默认中文字体优先：`PingFang SC`, `Microsoft YaHei`, `Noto Sans SC`, `sans-serif`。
- 主标题使用加粗字重，体现科技展板感。
- 英文小标题使用大写字母、较大字间距。

字号建议：

| 场景 | PC 字号 | 移动端字号 |
|---|---:|---:|
| 首页主标题 | 56px - 72px | 34px - 42px |
| 页面标题 | 40px - 48px | 28px - 34px |
| 卡片标题 | 26px - 34px | 22px - 26px |
| 正文 | 15px - 17px | 14px - 15px |
| 标签/辅助文字 | 12px - 14px | 12px |

### 6.4 卡片规范

- 大卡片圆角：24px - 32px。
- 小卡片圆角：12px - 18px。
- 边框：`1px solid rgba(30, 100, 220, 0.1)`。
- 阴影：蓝色轻阴影，避免黑色重阴影。
- 背景：半透明白色或极浅蓝渐变。

### 6.5 响应式规范

| 设备 | 宽度 | 布局 |
|---|---:|---|
| PC | >= 1200px | 内容最大宽度 1200px - 1440px，居中 |
| 平板 | 768px - 1199px | 双列或单列混合布局 |
| 手机 | < 768px | 单列布局，按钮加大，导航折叠 |

移动端要求：

- 顶部导航折叠为菜单按钮。
- 首页主标题适当缩小。
- 专业卡片单列堆叠。
- 作品列表单列展示。
- 评论输入框宽度占满。
- 点赞按钮和发送按钮高度不小于 40px。

---

## 7. 页面详细需求

### 7.1 首页封面页 `/`

#### 页面目标

展示毕设展主题，营造科技感第一印象，引导用户进入专业展区或热榜。

#### 页面结构

1. 顶部导航栏。
2. 学院 logo 与名称。
3. 主标题：智媒融合 · 创启未来。
4. 副标题：传媒技术学院 2026 届毕业设计展。
5. 展览简介文案。
6. 三个专业快捷入口。
7. 主按钮：进入展区。
8. 次按钮：查看热榜。
9. 右侧或背景 3D 科技立方体。
10. 底部数据波浪、光点、scroll 指示。

#### 展览简介文案建议

```text
本次毕业设计展以“智媒融合 · 创启未来”为主题，聚焦软件工程、电子信息工程、广播电视工程三大专业的创新成果与应用实践，展现未来媒介技术的无限潜力，共同推动传媒技术的创新发展与产业融合。
```

#### 交互

- 点击“进入展区”跳转 `/majors`。
- 点击专业入口跳转对应 `/major/:majorCode`。
- 点击“点赞热榜”跳转 `/ranking`。

---

### 7.2 专业展区页 `/majors`

#### 页面目标

展示三大专业方向，让用户快速理解专业主题并进入作品列表。

#### 页面结构

1. 顶部导航栏。
2. 页面标题：专业展区 / PROFESSIONAL ZONES。
3. 三张专业卡片。
4. 底部统计数据：3 大专业、60+ 优秀作品、300+ 毕业生、无限创意。

#### 专业卡片内容

| 专业 | 卡片编号 | 主题 | 副标题 | 描述 |
|---|---|---|---|---|
| 软件工程 | 01 | 数智焕新 | 数智驱动 · 焕新应用 | 聚焦软件开发、人工智能与数据应用等方向的创新实践。 |
| 电子信息工程 | 02 | 芯火智造 | 芯火相传 · 智造终端 | 聚焦嵌入式、物联网、通信与硬件系统的创新应用。 |
| 广播电视工程 | 03 | 虚实共生 | 虚实共生 · 视界新生 | 聚焦视音频制作、虚拟现实与数字媒体的融合创新。 |

#### 交互

- 点击“探索作品”进入对应专业作品列表。
- 卡片 hover 时轻微上浮，阴影增强。

---

### 7.3 专业作品列表页 `/major/:majorCode`

#### 页面目标

按专业展示作品，方便游客筛选、浏览、点赞和进入详情页。

#### 页面结构

1. 顶部导航栏。
2. 专业主题横幅。
3. 分类筛选标签。
4. 作品列表。
5. 分页器。

#### 软件工程横幅示例

```text
软件工程 / SOFTWARE ENGINEERING
数智焕新
数智驱动 · 焕新应用
以软件创新为引擎，融合智能技术与数据能力，打造更高效、更智能、更美好的应用体验。
```

#### 筛选标签示例

软件工程：

- 全部作品
- 人工智能
- 大数据
- 软件开发
- Web 应用
- 其他

电子信息工程：

- 全部作品
- 嵌入式
- 物联网
- 智能硬件
- 通信系统
- 其他

广播电视工程：

- 全部作品
- 虚拟制作
- 影像交互
- 数字媒体
- 智能媒资
- 其他

#### 作品卡片字段

| 字段 | 说明 |
|---|---|
| 作品 ID | 用于路由和接口调用 |
| 作品标题 | 例如：智管家 · 校园智能管理系统 |
| 专业 | 软件工程 / 电子信息工程 / 广播电视工程 |
| 标签 | Web 应用、项目实践等 |
| 简介 | 80 字以内 |
| 视频封面 | 图片 URL |
| 视频时长 | 例如 02:34 |
| 作者 | 可展示 1-3 人，多人时显示“等 3 人” |
| 点赞数 | 从后端接口获取 |
| 评论数 | 从后端接口获取 |

#### 交互

- 点击视频封面或作品标题进入作品详情页。
- 点击点赞按钮调用后端点赞接口。
- 点击评论图标进入详情页评论区。
- 点击分享图标复制详情页链接。
- 切换分类时前端过滤作品。
- 分页器在前端本地分页即可。

---

### 7.4 作品详情页 `/works/:workId`

#### 页面目标

展示单个作品的完整信息，并提供点赞与评论交互。

#### 页面结构

1. 顶部导航栏。
2. 面包屑：专业展区 / 软件工程 / 作品详情。
3. 专业横幅。
4. 视频展示区。
5. 作品信息区。
6. 点赞、评论、分享按钮。
7. 评论输入框。
8. 评论列表。
9. 页脚。

#### 作品详情字段

| 字段 | 说明 |
|---|---|
| workId | 作品唯一 ID |
| title | 作品标题 |
| major | 所属专业 |
| theme | 专业主题 |
| tags | 标签数组 |
| authors | 作者数组 |
| teacher | 指导老师 |
| coverUrl | 封面图 |
| videoUrl | 视频地址 |
| duration | 视频时长 |
| introduction | 作品简介 |
| likeCount | 点赞数 |
| commentCount | 评论数 |
| likedByMe | 当前游客是否已点赞 |

#### 评论区要求

- 评论输入框 placeholder：`说点什么吧...`
- 提交按钮文案：`发送`
- 评论按时间倒序展示。
- 评论用户显示为：`游客4821`、`游客7902`。
- 评论时间显示：`刚刚`、`1 小时前`、`2 天前`。
- 默认显示前 5-10 条评论，可点击“查看更多评论”。

#### 交互

- 点赞按钮点击后立即更新 UI，若接口失败则回滚。
- 评论提交成功后，新评论插入列表顶部。
- 评论提交失败时显示错误提示。
- 评论为空时禁止提交。
- 评论长度超过 200 字时禁止提交。
- 分享按钮复制当前页面链接。

---

### 7.5 点赞热榜页 `/ranking`

#### 页面目标

展示当前最受欢迎的作品，按点赞数排序。

#### 页面结构

1. 顶部导航栏。
2. 页面标题：点赞热榜 / HOT RANKING。
3. 榜单切换：今日热榜、本周热榜、总榜。
4. 前三名大卡片。
5. 第 4 名及以后列表。
6. 点赞数。
7. 详情按钮。
8. 页脚。

#### 榜单规则

- MVP 阶段可以只实现总榜。
- 今日热榜、本周热榜可以先保留 UI，接口可返回同一数据或预留参数。
- 默认按点赞数降序排序。
- 点赞数相同，按最近更新时间降序排序。
- 只展示点赞数大于 0 的作品。

#### 交互

- 点击“查看详情”进入作品详情页。
- 点击榜单 tab 重新请求数据。
- 榜单数据可每 30 秒刷新一次。

---

### 7.6 关于毕展页 `/about`

#### 页面目标

展示主办单位、联系方式、版权信息、学院介绍。

#### 页面结构

1. 顶部导航栏。
2. 学院与展览介绍。
3. 三大专业介绍。
4. 联系方式。
5. 二维码占位。
6. 版权信息。

---

## 8. 交互功能需求

### 8.1 游客身份识别

本项目无需登录，但需要识别同一游客，避免重复点赞和高频刷评论。

前端首次访问时生成匿名 `visitor_id`，存入 localStorage 和 cookie。

建议格式：

```text
visitor_id = crypto.randomUUID()
```

后端不要直接信任 visitor_id，需要结合请求地址和 User-Agent 生成服务端指纹。

服务端指纹建议：

```text
visitor_fingerprint = hash(work_id + visitor_id + ip + user_agent + server_salt)
```

注意：

- IP 地址必须由后端从请求中获取。
- 若部署在 Nginx 或反向代理之后，需要正确读取可信的 `X-Forwarded-For` 或 `X-Real-IP`。
- IP 不要明文存储，应存储哈希值。

---

### 8.2 点赞功能

#### 功能说明

游客可以对作品点赞。每个游客对同一个作品只能有一个有效点赞。

#### 业务规则

1. 游客无需登录。
2. 点赞时后端获取用户地址/IP。
3. 后端结合 visitor_id、IP、User-Agent 生成指纹。
4. 同一个作品下，同一个指纹只能存在一条点赞记录。
5. 如果用户未点赞，点击后创建或激活点赞记录。
6. 如果用户已点赞，再次点击后取消点赞。
7. 点赞数实时返回给前端。
8. 点赞状态会影响热榜排序。

#### 前端表现

未点赞：

- 灰色或描边点赞图标。
- 文案显示点赞数。

已点赞：

- 蓝色高亮点赞图标。
- 点赞数 +1。

失败情况：

- 请求失败时恢复原状态。
- 显示提示：`操作失败，请稍后再试`。

---

### 8.3 评论功能

#### 功能说明

游客可以对作品发表评论。评论时不需要输入昵称，后端自动生成游客昵称和评论 ID。

#### 业务规则

1. 游客无需登录。
2. 用户提交评论时，后端获取用户地址/IP。
3. 后端生成随机评论 ID。
4. 后端生成随机游客昵称，例如：`游客4821`。
5. 评论内容存入数据库。
6. 评论默认状态为正常。
7. 评论列表按创建时间倒序展示。
8. 评论需要进行基础安全检测和反刷限制。

#### 评论校验

| 校验项 | 规则 |
|---|---|
| 是否为空 | 空内容不可提交 |
| 最大长度 | 200 字以内 |
| 最小长度 | 1 字以上 |
| HTML 标签 | 服务端转义或清除 |
| 敏感词 | 命中后拒绝或置为待审核 |
| 高频提交 | 触发限流后拒绝 |

#### 前端提示

| 场景 | 提示 |
|---|---|
| 空评论 | 请输入评论内容 |
| 超长 | 评论最多 200 字 |
| 成功 | 评论已发布 |
| 限流 | 操作太频繁，请稍后再试 |
| 敏感词 | 评论包含不合适内容，请修改后再提交 |

---

### 8.4 评论点赞功能

MVP 阶段不强制实现评论点赞。

如实现，规则与作品点赞类似：每个游客对单条评论只能点赞一次。

---

### 8.5 分享功能

前端实现即可。

规则：

- 点击分享按钮复制作品详情页 URL。
- 成功提示：`链接已复制`。
- 失败时提示：`复制失败，请手动复制链接`。

---

## 9. 后端 API 设计

### 9.1 API 统一规范

基础路径：

```text
/api
```

统一响应格式：

```json
{
  "code": 0,
  "message": "ok",
  "data": {}
}
```

错误响应格式：

```json
{
  "code": 40001,
  "message": "操作太频繁，请稍后再试",
  "data": null
}
```

### 9.2 错误码

| code | 含义 |
|---:|---|
| 0 | 成功 |
| 40000 | 参数错误 |
| 40001 | 操作太频繁 |
| 40002 | 重复操作 |
| 40003 | 内容不合法 |
| 40400 | 资源不存在 |
| 50000 | 服务器错误 |

---

### 9.3 健康检查

#### `GET /api/health`

响应：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "status": "ok"
  }
}
```

---

### 9.4 获取作品交互统计

#### `GET /api/interactions/summary?work_ids=w001,w002,w003`

说明：

前端作品数据可以是静态配置，但点赞数、评论数、当前用户是否点赞需要从后端获取。

请求参数：

| 参数 | 类型 | 必填 | 说明 |
|---|---|---:|---|
| work_ids | string | 是 | 逗号分隔的作品 ID |

请求头：

```text
X-Visitor-Id: 前端生成的 visitor_id
```

响应：

```json
{
  "code": 0,
  "message": "ok",
  "data": [
    {
      "work_id": "w001",
      "like_count": 126,
      "comment_count": 18,
      "liked_by_me": true
    }
  ]
}
```

---

### 9.5 获取单个作品交互状态

#### `GET /api/works/:workId/interaction`

响应：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "work_id": "w001",
    "like_count": 126,
    "comment_count": 18,
    "liked_by_me": true
  }
}
```

---

### 9.6 切换点赞状态

#### `POST /api/works/:workId/like/toggle`

请求头：

```text
X-Visitor-Id: 前端生成的 visitor_id
```

请求体：

```json
{
  "client_ts": 1730000000000
}
```

说明：

- 后端从请求中获取 IP。
- 后端读取 User-Agent。
- 后端结合 visitor_id、IP、User-Agent 和 server_salt 生成指纹。
- 若未点赞，则点赞。
- 若已点赞，则取消点赞。

响应：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "work_id": "w001",
    "liked": true,
    "like_count": 127
  }
}
```

---

### 9.7 获取评论列表

#### `GET /api/works/:workId/comments?page=1&page_size=10`

请求参数：

| 参数 | 类型 | 默认值 | 说明 |
|---|---|---:|---|
| page | number | 1 | 页码 |
| page_size | number | 10 | 每页数量，最大 50 |

响应：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "items": [
      {
        "id": "cmt_7fa3d9",
        "work_id": "w001",
        "visitor_name": "游客4821",
        "content": "界面设计很简洁，交互也很流畅，期待后续优化！",
        "created_at": "2026-05-01T10:00:00Z"
      }
    ],
    "page": 1,
    "page_size": 10,
    "total": 18
  }
}
```

---

### 9.8 提交评论

#### `POST /api/works/:workId/comments`

请求头：

```text
X-Visitor-Id: 前端生成的 visitor_id
```

请求体：

```json
{
  "content": "界面设计很简洁，交互也很流畅，期待后续优化！",
  "client_ts": 1730000000000,
  "website": ""
}
```

字段说明：

| 字段 | 说明 |
|---|---|
| content | 评论内容 |
| client_ts | 客户端时间戳，用于辅助反刷 |
| website | 蜜罐字段，前端隐藏，正常用户为空 |

响应：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "id": "cmt_7fa3d9",
    "work_id": "w001",
    "visitor_name": "游客4821",
    "content": "界面设计很简洁，交互也很流畅，期待后续优化！",
    "created_at": "2026-05-01T10:00:00Z"
  }
}
```

---

### 9.9 获取点赞热榜

#### `GET /api/rankings/likes?range=all&page=1&page_size=10`

请求参数：

| 参数 | 类型 | 默认值 | 说明 |
|---|---|---:|---|
| range | string | all | all / week / today |
| page | number | 1 | 页码 |
| page_size | number | 10 | 每页数量 |

响应：

```json
{
  "code": 0,
  "message": "ok",
  "data": {
    "items": [
      {
        "rank": 1,
        "work_id": "w001",
        "like_count": 2568
      }
    ],
    "page": 1,
    "page_size": 10,
    "total": 30
  }
}
```

说明：

后端只需要返回 work_id 和 like_count，前端根据本地作品配置补充作品标题、专业、封面、简介等信息。

---

## 10. 数据库设计

数据库建议使用 PostgreSQL。Rust 后端建议使用 sqlx 管理数据库连接和 migrations。

### 10.1 likes 点赞表

```sql
CREATE TABLE likes (
  id UUID PRIMARY KEY,
  work_id VARCHAR(64) NOT NULL,
  visitor_fingerprint_hash VARCHAR(128) NOT NULL,
  visitor_id_hash VARCHAR(128),
  ip_hash VARCHAR(128) NOT NULL,
  user_agent_hash VARCHAR(128),
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE (work_id, visitor_fingerprint_hash)
);

CREATE INDEX idx_likes_work_active ON likes(work_id, is_active);
CREATE INDEX idx_likes_updated_at ON likes(updated_at);
```

说明：

- `is_active = true` 表示当前有效点赞。
- 取消点赞时不要删除记录，更新为 `false`。
- 再次点赞时更新为 `true`。
- `UNIQUE (work_id, visitor_fingerprint_hash)` 用于限制同一游客对同一作品重复点赞。

---

### 10.2 comments 评论表

```sql
CREATE TABLE comments (
  id UUID PRIMARY KEY,
  public_id VARCHAR(64) NOT NULL UNIQUE,
  work_id VARCHAR(64) NOT NULL,
  visitor_name VARCHAR(64) NOT NULL,
  visitor_fingerprint_hash VARCHAR(128) NOT NULL,
  ip_hash VARCHAR(128) NOT NULL,
  user_agent_hash VARCHAR(128),
  content TEXT NOT NULL,
  status VARCHAR(20) NOT NULL DEFAULT 'visible',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_comments_work_created ON comments(work_id, created_at DESC);
CREATE INDEX idx_comments_status ON comments(status);
```

status 取值：

| status | 说明 |
|---|---|
| visible | 正常展示 |
| pending | 待审核，MVP 可不使用 |
| hidden | 隐藏 |
| rejected | 拒绝 |

---

### 10.3 abuse_logs 风控日志表

```sql
CREATE TABLE abuse_logs (
  id UUID PRIMARY KEY,
  event_type VARCHAR(64) NOT NULL,
  work_id VARCHAR(64),
  visitor_fingerprint_hash VARCHAR(128),
  ip_hash VARCHAR(128),
  reason TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_abuse_logs_created ON abuse_logs(created_at DESC);
CREATE INDEX idx_abuse_logs_event_type ON abuse_logs(event_type);
```

---

## 11. 反刷与安全策略

### 11.1 点赞反刷

| 风险 | 策略 |
|---|---|
| 同一游客重复点赞 | 数据库唯一索引限制 |
| 高频切换点赞 | Redis 限流，单指纹每分钟最多 10 次 |
| 单 IP 大量点赞 | 单 IP 每分钟最多 60 次 |
| 脚本请求 | 校验 User-Agent、Referer、请求频率 |
| 伪造 visitor_id | 后端结合 IP 和 UA 生成服务端指纹 |

### 11.2 评论反刷

| 风险 | 策略 |
|---|---|
| 高频评论 | 单指纹 30 秒最多 1 条，每小时最多 10 条 |
| 单 IP 大量评论 | 单 IP 每分钟最多 5 条 |
| 机器提交 | 蜜罐字段 website 必须为空 |
| 敏感词 | 命中敏感词拒绝或 pending |
| XSS | 服务端转义或清除 HTML 标签 |
| 超长评论 | 最大 200 字 |
| 空评论 | 不允许提交 |

### 11.3 IP 处理

要求：

- 不要将真实 IP 明文存入数据库。
- 使用服务端盐值进行哈希。
- 哈希示例：`sha256(ip + server_salt)`。
- `server_salt` 存放在环境变量中。

### 11.4 CORS

开发环境允许：

```text
http://localhost:5173
http://localhost:3000
```

生产环境只允许正式域名。

### 11.5 XSS 防护

后端必须对评论内容进行处理：

- 去除 HTML 标签，或转义 `<`, `>`, `&`, `"`, `'`。
- 前端展示评论时不要使用 `v-html`。
- 前端使用普通文本节点渲染评论。

---

## 12. 前端技术要求

### 12.1 技术栈

- Vben Admin。
- Vue 3。
- TypeScript。
- Vite。
- Pinia。
- Vue Router。
- Axios 或项目内置 request 工具。
- UnoCSS / Tailwind / Less 均可，优先遵循 Vben 项目默认规范。

### 12.2 前端目录建议

```text
frontend/src/
├── api/
│   ├── interactions.ts
│   └── rankings.ts
├── assets/
│   ├── images/
│   └── icons/
├── components/
│   ├── exhibition/
│   │   ├── ExhibitionHeader.vue
│   │   ├── ExhibitionFooter.vue
│   │   ├── TechHero.vue
│   │   ├── MajorCard.vue
│   │   ├── WorkCard.vue
│   │   ├── LikeButton.vue
│   │   ├── CommentPanel.vue
│   │   └── RankingList.vue
├── data/
│   ├── majors.ts
│   └── works.ts
├── router/
│   └── routes/
│       └── exhibition.ts
├── stores/
│   ├── visitor.ts
│   ├── interactions.ts
│   └── ranking.ts
├── styles/
│   ├── exhibition.less
│   └── variables.less
└── views/
    └── exhibition/
        ├── Home.vue
        ├── Majors.vue
        ├── MajorWorks.vue
        ├── WorkDetail.vue
        ├── Ranking.vue
        └── About.vue
```

### 12.3 静态作品数据

前端先使用静态数据定义作品。示例：

```ts
export interface WorkItem {
  id: string;
  title: string;
  majorCode: 'software' | 'electronic' | 'broadcast';
  majorName: string;
  theme: string;
  tags: string[];
  authors: string[];
  teacher: string;
  coverUrl: string;
  videoUrl: string;
  duration: string;
  introduction: string;
}
```

### 12.4 前端状态管理

#### visitor store

负责生成和保存匿名 visitor_id。

字段：

```ts
visitorId: string
```

方法：

```ts
initVisitorId()
getVisitorId()
```

#### interactions store

负责作品点赞、评论数、当前用户点赞状态。

字段：

```ts
interactionMap: Record<string, {
  likeCount: number;
  commentCount: number;
  likedByMe: boolean;
}>
```

方法：

```ts
fetchSummary(workIds: string[])
toggleLike(workId: string)
```

### 12.5 请求封装

所有交互接口都需要带上请求头：

```text
X-Visitor-Id: visitor_id
```

失败时统一处理：

- 40001：提示操作太频繁。
- 40003：提示内容不合法。
- 50000：提示服务器异常。

---

## 13. 后端技术要求

### 13.1 技术栈

- Rust stable。
- axum。
- tokio。
- serde。
- sqlx。
- PostgreSQL。
- tower-http。
- tracing。
- uuid。
- chrono。
- sha2。
- validator。
- redis，可选但建议用于限流。

### 13.2 后端目录建议

```text
backend/
├── Cargo.toml
├── .env.example
├── migrations/
│   ├── 0001_create_likes.sql
│   ├── 0002_create_comments.sql
│   └── 0003_create_abuse_logs.sql
└── src/
    ├── main.rs
    ├── config.rs
    ├── db.rs
    ├── error.rs
    ├── response.rs
    ├── routes/
    │   ├── mod.rs
    │   ├── health.rs
    │   ├── likes.rs
    │   ├── comments.rs
    │   ├── interactions.rs
    │   └── rankings.rs
    ├── models/
    │   ├── like.rs
    │   ├── comment.rs
    │   └── ranking.rs
    ├── services/
    │   ├── fingerprint.rs
    │   ├── rate_limit.rs
    │   ├── sanitize.rs
    │   ├── like_service.rs
    │   ├── comment_service.rs
    │   └── ranking_service.rs
    └── utils/
        ├── ip.rs
        └── random_name.rs
```

### 13.3 环境变量

`.env.example`：

```env
APP_HOST=0.0.0.0
APP_PORT=8080
DATABASE_URL=postgres://postgres:postgres@localhost:5432/graduation_exhibition
REDIS_URL=redis://localhost:6379
SERVER_SALT=please-change-this-salt
CORS_ALLOWED_ORIGINS=http://localhost:5173,http://localhost:3000
TRUST_PROXY=true
```

### 13.4 axum 路由

```rust
/api/health
/api/interactions/summary
/api/works/:work_id/interaction
/api/works/:work_id/like/toggle
/api/works/:work_id/comments
/api/rankings/likes
```

### 13.5 服务端核心逻辑

#### 点赞 toggle 逻辑

伪代码：

```text
1. 读取 work_id。
2. 读取 X-Visitor-Id。
3. 获取 IP 和 User-Agent。
4. 生成 visitor_fingerprint_hash。
5. 检查限流。
6. 查询 likes 表是否已有记录。
7. 如果没有记录，插入 is_active = true。
8. 如果已有记录且 is_active = true，更新为 false。
9. 如果已有记录且 is_active = false，更新为 true。
10. 统计当前 work_id 的有效点赞数。
11. 返回 liked 状态和 like_count。
```

#### 评论提交逻辑

伪代码：

```text
1. 读取 work_id。
2. 读取 content。
3. 校验 content 非空且长度 <= 200。
4. 检查蜜罐字段 website 是否为空。
5. 获取 IP 和 User-Agent。
6. 生成 visitor_fingerprint_hash。
7. 检查评论限流。
8. 清理或转义 content。
9. 检查敏感词。
10. 生成 public_id。
11. 生成 visitor_name，如 游客4821。
12. 插入 comments 表。
13. 返回新评论。
```

---

## 14. Docker 与启动要求

### 14.1 docker-compose.yml

需要包含：

- PostgreSQL。
- Redis。
- backend 服务，可选。

示例端口：

| 服务 | 端口 |
|---|---:|
| frontend | 5173 |
| backend | 8080 |
| PostgreSQL | 5432 |
| Redis | 6379 |

### 14.2 README 需要包含

1. 项目介绍。
2. 技术栈。
3. 环境要求。
4. 前端启动命令。
5. 后端启动命令。
6. 数据库初始化命令。
7. API 简要说明。
8. 目录结构说明。

---

## 15. 示例数据

### 15.1 专业数据

```ts
export const majors = [
  {
    code: 'software',
    name: '软件工程',
    theme: '数智焕新',
    subtitle: '数智驱动 · 焕新应用',
    description: '聚焦软件开发、人工智能与数据应用等方向的创新实践。'
  },
  {
    code: 'electronic',
    name: '电子信息工程',
    theme: '芯火智造',
    subtitle: '芯火相传 · 智造终端',
    description: '聚焦嵌入式、物联网、通信与硬件系统的创新应用。'
  },
  {
    code: 'broadcast',
    name: '广播电视工程',
    theme: '虚实共生',
    subtitle: '虚实共生 · 视界新生',
    description: '聚焦视音频制作、虚拟现实与数字媒体的融合创新。'
  }
];
```

### 15.2 作品数据示例

```ts
export const works = [
  {
    id: 'w001',
    title: '智管家 · 校园智能管理系统',
    majorCode: 'software',
    majorName: '软件工程',
    theme: '数智焕新',
    tags: ['智慧校园', 'Web应用', '项目实践'],
    authors: ['张同学', '李同学', '王同学'],
    teacher: '王老师',
    coverUrl: '/images/works/software-001.jpg',
    videoUrl: '/videos/software-001.mp4',
    duration: '02:34',
    introduction: '系统整合校园多场景数据，提供设备监控、行为分析与预警、资源调度优化一体化管理能力，助力校园治理智能化升级。'
  },
  {
    id: 'w002',
    title: '云舟 · 低代码开发平台',
    majorCode: 'software',
    majorName: '软件工程',
    theme: '数智焕新',
    tags: ['低代码', 'Web应用', '项目实践'],
    authors: ['陈同学', '刘同学'],
    teacher: '李老师',
    coverUrl: '/images/works/software-002.jpg',
    videoUrl: '/videos/software-002.mp4',
    duration: '01:58',
    introduction: '一款可视化低代码开发平台，支持拖拽式组件编辑、流程引擎与权限管理，帮助开发者快速构建企业级应用。'
  }
];
```

---

## 16. 验收标准

### 16.1 前端验收

| 验收项 | 标准 |
|---|---|
| 首页 | 视觉风格贴近原型图，主标题、专业入口、按钮完整 |
| 专业展区 | 三个专业卡片完整展示，点击可进入作品列表 |
| 作品列表 | 可按专业展示作品，点赞数和评论数正常加载 |
| 作品详情 | 视频、简介、作者、指导老师、点赞、评论完整 |
| 热榜页 | 能展示点赞排行并跳转详情 |
| 响应式 | PC、平板、手机均可正常浏览 |
| 状态反馈 | 点赞、评论、失败提示清晰 |
| 安全渲染 | 评论内容不使用 v-html |

### 16.2 后端验收

| 验收项 | 标准 |
|---|---|
| 健康检查 | `/api/health` 正常返回 |
| 点赞 | 同一游客同一作品只能有一个有效点赞 |
| 取消点赞 | 再次点击可取消点赞 |
| 点赞数 | 点赞数统计准确 |
| 评论 | 可提交评论并写入数据库 |
| 评论列表 | 按时间倒序返回 |
| 评论 ID | 每条评论有唯一 public_id |
| 游客昵称 | 自动生成游客昵称 |
| IP 处理 | 不明文存储 IP |
| 限流 | 高频点赞和评论会被拒绝 |
| 热榜 | 按点赞数降序返回 |

### 16.3 联调验收

| 验收项 | 标准 |
|---|---|
| 前端调用后端 | 所有 API 可正常请求 |
| visitor_id | 前端可生成并在请求头传递 |
| likedByMe | 刷新页面后仍能识别是否点赞 |
| 评论提交 | 提交后列表立即展示 |
| 热榜同步 | 点赞后热榜数据可更新 |
| 错误提示 | 后端错误码能被前端正确展示 |

---

## 17. 开发优先级

### P0：必须完成

1. 项目初始化。
2. 前端 5 个核心页面。
3. 静态作品数据。
4. 点赞 toggle 接口。
5. 评论提交接口。
6. 评论列表接口。
7. 点赞热榜接口。
8. 数据库 migrations。
9. 基础限流。
10. 响应式适配。

### P1：建议完成

1. 首页动效。
2. 卡片 hover 动效。
3. 评论查看更多。
4. 榜单 tab。
5. 分享复制链接。
6. Redis 限流。
7. Docker Compose。

### P2：后续扩展

1. 管理后台。
2. 评论审核。
3. 作品在线配置。
4. WebSocket 实时热榜。
5. 上传作品视频。
6. 数据统计看板。

---

## 18. Claude Code 生成要求总结

请 Claude Code 按以下要求实现：

1. 生成完整 monorepo 项目。
2. 前端目录为 `frontend`，后端目录为 `backend`。
3. 前端使用 Vben Admin、Vue 3、TypeScript。
4. 后端使用 Rust axum。
5. 数据库使用 PostgreSQL。
6. Redis 用于限流，若时间不足可以先用内存限流并预留 Redis 接口。
7. 作品数据先放在前端静态 `works.ts`。
8. 后端只存储点赞、评论和风控日志。
9. 所有接口返回统一 JSON 格式。
10. 评论内容必须做长度校验和 XSS 处理。
11. 点赞和评论必须基于 IP、User-Agent、visitor_id 做匿名指纹。
12. 不实现登录注册。
13. 不实现后台管理。
14. README 必须写清楚如何启动前端、后端、数据库。
15. 代码需要有合理注释，便于后续继续开发。

---

## 19. 最终交付物

Claude Code 最终应生成：

```text
media-tech-graduation-exhibition/
├── frontend/                  # 可运行的前端项目
├── backend/                   # 可运行的 Rust axum 后端项目
├── docs/
│   └── api.md                 # API 文档
├── docker-compose.yml
├── README.md
└── .gitignore
```

前端运行后，至少可以访问：

```text
/
/majors
/major/software
/major/electronic
/major/broadcast
/works/w001
/ranking
/about
```

后端运行后，至少可以访问：

```text
GET  /api/health
GET  /api/interactions/summary
GET  /api/works/:workId/interaction
POST /api/works/:workId/like/toggle
GET  /api/works/:workId/comments
POST /api/works/:workId/comments
GET  /api/rankings/likes
```

