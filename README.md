# Studio Board

Studio Board 是一个面向创意制作团队的协作平台，用于管理制作板块、执行卡片和阶段进度。项目基于 Rust、Axum、Askama 和 SQLite 构建，包含仪表盘、板块列表、详情页、新建页面和接口层，适合作为轻量级制作协同系统的基础工程。

## 适用场景

- 创意团队按项目或工序维护制作板块。
- 制作负责人管理卡片状态、优先级和到期时间。
- 管理者通过首页掌握当前板块规模与执行负载。

## 核心能力

- 提供首页仪表盘，汇总板块数量和事项总数。
- 支持浏览全部制作板块并进入详情页查看卡片内容。
- 提供新建制作卡页面，便于补充阶段与说明信息。
- 提供 `/api/dashboard`、`/api/boards`、`/api/boards/:code` 等接口。
- 提供静态资源目录和模板目录，方便继续做界面扩展。
- 采用 SQLite 持久化，便于在容器中快速启动。

## 技术栈

- 后端：Rust
- Web 框架：Axum
- 模板引擎：Askama
- 数据存储：SQLite
- 静态资源：原生 CSS
- 运行方式：Docker、本地 Cargo

## 目录结构

- `src/main.rs`：应用入口与路由注册。
- `src/routes/`：页面与 API 路由。
- `src/services/`：仪表盘与板块业务服务。
- `src/db.rs`、`src/seed.rs`：数据库初始化与种子数据。
- `templates/`：Askama 模板文件。
- `static/`：静态样式资源。
- `data/`：SQLite 数据目录。
- `patch/`：预留补丁目录。

## 启动方式

### 使用 Docker

```bash
docker build -t studio-board .
docker run --rm -p 8080:8080 studio-board
```

### 本地开发

```bash
cargo run
```

默认监听端口为 `8080`。

## 默认账号

- 用户名：`mara`
- 密码：`studio123`

## 后续扩展方向

- 增加素材清单、工序排期和多项目切换能力。
- 增加评论、附件和审阅状态流转。
- 接入操作日志、消息提醒和报表导出。
