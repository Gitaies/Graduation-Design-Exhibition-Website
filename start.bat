@echo off
setlocal enabledelayedexpansion

echo ==========================================
echo Media Tech Graduation Exhibition 2026
echo Quick Start Script (Windows)
echo ==========================================
echo.

REM Check Docker
where docker >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Docker not installed
    echo Please install Docker Desktop: https://docs.docker.com/desktop/install/windows-install/
    pause
    exit /b 1
)

where docker-compose >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Docker Compose not installed
    pause
    exit /b 1
)

echo [OK] Docker environment check passed
echo.

REM 1. Start Docker services
echo [1/5] Starting MySQL and Redis...
docker-compose up -d

echo Waiting for MySQL to start (15 seconds)...
timeout /t 15 /nobreak >nul

REM 2. Check MySQL readiness
echo Checking MySQL connection...
:wait_mysql
docker exec media-tech-mysql mysql -uroot -ppassword -e "SELECT 1" >nul 2>nul
if %errorlevel% neq 0 (
    echo Waiting for MySQL...
    timeout /t 2 /nobreak >nul
    goto wait_mysql
)
echo [OK] MySQL is ready
echo.

REM 3. Import works data
echo [2/5] Importing works data...
if exist "backend\backend\import_works.sql" (
    docker exec -i media-tech-mysql mysql -uroot -ppassword graduation_exhibition < backend\backend\import_works.sql
    echo [OK] Works data imported successfully
) else (
    echo [WARNING] backend\backend\import_works.sql not found
)
echo.

REM 4. Check backend environment
echo [3/5] Checking backend configuration...
if not exist "backend\.env" (
    echo [WARNING] backend\.env does not exist, using .env.example
    if exist "backend\.env.example" (
        copy backend\.env.example backend\.env >nul
    )
)
echo [OK] Backend configuration checked
echo.

REM 5. Check frontend environment
echo [4/5] Checking frontend configuration...
if not exist "frontend\.env" (
    echo Creating frontend\.env...
    echo NUXT_PUBLIC_API_BASE=http://localhost:8080/api > frontend\.env
)
echo [OK] Frontend configuration checked
echo.

REM 6. Display startup information
echo ==========================================
echo [OK] Environment setup complete!
echo ==========================================
echo.
echo Next steps:
echo.
echo 1. Start backend (new terminal):
echo    cd backend
echo    cargo run
echo    Backend URL: http://localhost:8080
echo.
echo 2. Start frontend (new terminal):
echo    cd frontend
echo    pnpm install  # first time only
echo    pnpm dev
echo    Frontend URL: http://localhost:3000
echo.
echo 3. Visit website:
echo    http://localhost:3000
echo.
echo ==========================================
echo Management commands:
echo ==========================================
echo.
echo View Docker logs:
echo   docker-compose logs -f
echo.
echo Stop services:
echo   docker-compose down
echo.
echo Reset database (delete all data):
echo   docker-compose down -v
echo.
echo ==========================================
echo.
pause
