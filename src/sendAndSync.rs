// rust语言本身不限定并发模型
// 其并发的基础来自 std::mark提供的两个trait： Send Sync
// Send允许在不同线程间转移转移所有权
// Sync允许多线程访问

// 如果一个引用实现了Send，其值类型就实现了Sync