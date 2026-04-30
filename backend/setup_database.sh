#!/bin/bash

# 数据库设置脚本
# 使用方法: ./setup_database.sh [mysql_root_password]

MYSQL_PASSWORD=${1:-""}

if [ -z "$MYSQL_PASSWORD" ]; then
    echo "请输入 MySQL root 密码:"
    read -s MYSQL_PASSWORD
fi

echo "正在创建数据库..."

# 创建数据库
mysql -u root -p"$MYSQL_PASSWORD" <<EOF
CREATE DATABASE IF NOT EXISTS graduation_exhibition CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE graduation_exhibition;
EOF

if [ $? -ne 0 ]; then
    echo "❌ 数据库创建失败，请检查 MySQL 密码是否正确"
    exit 1
fi

echo "✅ 数据库创建成功"
echo "正在运行迁移脚本..."

# 按顺序运行迁移脚本
for migration in migrations/*.sql; do
    echo "运行 $migration..."
    mysql -u root -p"$MYSQL_PASSWORD" graduation_exhibition < "$migration"
    if [ $? -ne 0 ]; then
        echo "❌ 迁移失败: $migration"
        exit 1
    fi
done

echo "✅ 所有迁移脚本执行成功"
echo ""
echo "下一步："
echo "1. 更新 .env 文件中的 DATABASE_URL，将密码改为你的实际密码"
echo "2. 确保 Redis 服务正在运行"
echo "3. 运行 cargo run 启动服务器"
