## 开发语言与包管理器
- 开发语言 `Rust 1.49.0`
- 包管理器 `Cargo`
## 项目构建
1. clone 项目到本地：`git clone https://github.com/caitlingao/geekbang_task_demo.git`
2. 构造项目：`cargo build`
3. 运行项目：`cargo run xxx`，具体运行命令参见下面`运行命令`内容
## 运行命令
- 添加 Todo 项，数据存储在 tmp/tasks.json 文件中 
```
cargo run -- todo --add "first task"
```
- 完成 Todo 项 
```
cargo run -- todo --done 1
```
- 查看 Todo 列表，缺少情况 
```
cargo run -- todo --list
```
- 查看 Todo 列表，使用 all 参数 
```
cargo run -- todo --list --all
```
- 用户登录
```
cargo run -- login -u "example001@example.com" // 执行此命令后敲回车，等待提示输入密码

Password: // 上一步敲回车后，在屏幕上会显示这样，输入密码回车。密码：123456
```
- 用户退出 
```
cargo run -- logout
```
- 下载文件，下载文件会被存入 tmp/download/tasks.json 文件中
```
cargo run -- todo --export -t todolist
```
- 导入文件，文件格式只支持 .json 格式，文件中数据格式如下：
```
cargo run -- todo --import -f files/tasks.json

// 待导入文件中数据格式
[{"id":1,"content":"first list","finished":false,"user_id":1,"created_at":"2021-01-05T08:58:22.928983Z","updated_at":"2021-01-05T08:58:36.712812Z"}]
```
- 初始化数据库
```
cargo run -- todo --init
```
## 项目文件说明
- 代码文件 `src` 目录下
- 开发分析设计文档 `docs` 目录下
- database `Postgres`

## 使用数据库持久化数据
- 重命名`.env.sample` 为 `.env`，修改文件中数据库配置信息；
- 命令: 
```
diesel setup // 创建数据库
```
```
diesel migration run // 执行 migration 迁移文件
```