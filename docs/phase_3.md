## 第三阶段：支持列表导入和导出开发分析设计
### 实现思路
1. src/databases/task.rs 文件中增加 export_tasks 方法，导出已有数据
```Rust
let download_tasks: Vec<Task> = tasks
    .iter()
    .filter(|task| task.user_id == user_id)
    .cloned()
    .collect();

let json: String = serde_json::to_string(&download_tasks)?;
fs::write(&download_path, &json).expect("Unable write to file");
```
2. src/databases/task.rs 文件中增加 import_tasks 方法，导入数据
```Rust
for waiting_task in waiting_tasks.iter() {
    // 如果两条 task 的 id, user_id, content 同时相同，即认为是重复数据，不执行导入
    if tasks
        .iter()
        .any(|task| task.id == waiting_task.id && task.user_id == waiting_task.user_id && task.content == waiting_task.content) {
        fail_count += 1;
        continue;
    }
    success_count += 1;
    tasks.push(waiting_task.clone());
}

let json: String = serde_json::to_string(&tasks)?;
fs::write(&task_file_path, &json).expect("Unable write to file");
```
3. 相关细节
- 用户登录后才可进行数据的导入导出操作；
- 导出数据时，只可导出当前登录用户创建数据；
- 导入数据时，对数据要进行去重，去重条件 id, user_id, content 三个字段同时相，即认为是同一条数据；