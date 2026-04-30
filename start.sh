#!/bin/bash

# 传媒技术学院 2026 届毕业设计展网站 - 快速启动脚本

set -e

echo "=========================================="
echo "传媒技术学院 2026 届毕业设计展网站"
echo "快速启动脚本"
echo "=========================================="
echo ""

# 检查 Docker
if ! command -v docker &> /dev/null; then
    echo "❌ 错误: 未安装 Docker"
    echo "请先安装 Docker: https://docs.docker.com/get-docker/"
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    echo "❌ 错误: 未安装 Docker Compose"
    echo "请先安装 Docker Compose"
    exit 1
fi

echo "✅ Docker 环境检查通过"
echo ""

# 1. 启动 Docker 服务
echo "📦 步骤 1/5: 启动 MySQL 和 Redis..."
docker-compose up -d

echo "⏳ 等待 MySQL 启动完成（15秒）..."
sleep 15

# 2. 检查 MySQL 是否就绪
echo "🔍 检查 MySQL 连接..."
until docker exec media-tech-mysql mysql -uroot -ppassword -e "SELECT 1" &> /dev/null; do
    echo "⏳ 等待 MySQL 就绪..."
    sleep 2
done
echo "✅ MySQL 已就绪"
echo ""

# 3. 导入作品数据
echo "📊 步骤 2/5: 导入作品数据..."
if [ -f "backend/import_works.sql" ]; then
    docker exec -i media-tech-mysql mysql -uroot -ppassword graduation_exhibition < backend/import_works.sql
    echo "✅ 作品数据导入成功"
else
    echo "⚠️  警告: 未找到 backend/import_works.sql"
fi
echo ""

# 4. 检查后端环境变量
echo "⚙️  步骤 3/5: 检查后端配置..."
if [ ! -f "backend/.env" ]; then
    echo "⚠️  警告: backend/.env 不存在，使用 .env.example"
    cp backend/.env.example backend/.env
fi
echo "✅ 后端配置检查完成"
echo ""

# 5. 检查前端环境变量
echo "⚙️  步骤 4/5: 检查前端配置..."
if [ ! -f "frontend/.env" ]; then
    echo "创建 frontend/.env..."
    echo "NUXT_PUBLIC_API_BASE=http://localhost:8080/api" > frontend/.env
fi
echo "✅ 前端配置检查完成"
echo ""

# 6. 显示启动信息
echo "=========================================="
echo "✅ 环境准备完成！"
echo "=========================================="
echo ""
echo "📝 下一步操作："
echo ""
echo "1️⃣  启动后端（新终端）："
echo "   cd backend"
echo "   cargo run"
echo "   后端地址: http://localhost:8080"
echo ""
echo "2️⃣  启动前端（新终端）："
echo "   cd frontend"
echo "   pnpm install  # 首次运行需要"
echo "   pnpm dev"
echo "   前端地址: http://localhost:3000"
echo ""
echo "3️⃣  访问网站："
echo "   http://localhost:3000"
echo ""
echo "=========================================="
echo "🔧 管理命令："
echo "=========================================="
echo ""
echo "查看 Docker 日志:"
echo "  docker-compose logs -f"
echo ""
echo "停止服务:"
echo "  docker-compose down"
echo ""
echo "重置数据库（删除所有数据）:"
echo "  docker-compose down -v"
echo ""
echo "=========================================="
