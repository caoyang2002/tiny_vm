# TinyVM

[English](../README.md) | **简体中文**

一个基于MVP的栈式字节码虚拟机

这个虚拟机运行一个简单、图灵完备的指令集。

在大约250行代码（LOC）和大量注释的情况下，它旨在易于理解，并且可以轻松地在除Rust之外的其他语言中复现。

它还包括一个基本的编译器，可以执行`test_files`文件夹中的程序。

## 指令

| 指令             | 描述                                                                                       |
|------------------|--------------------------------------------------------------------------------------------|
| Push (isize)     | 将参数推到栈顶                                                                               |
| Pop              | 移除栈顶的值                                                                               |
| Add              | 弹出栈顶的两个值并将它们的和推回栈顶                                                   |
| Sub              | 弹出栈顶的两个值并将它们的差推回栈顶                                                   |
| Mul              | 弹出栈顶的两个值并将它们的积推回栈顶                                                   |
| Div              | 弹出栈顶的两个值并将它们的商推回栈顶                                                   |
| Jump (label)    | 将指令指针设置为标签                                                                 |
| JNE  (label)    | 如果栈顶非零则跳转                                                                       |
| JE   (label)    | 如果栈顶为零则跳转                                                                       |
| JGT  (label)    | 如果栈顶大于零则跳转                                                                     |
| JLT  (label)    | 如果栈顶小于零则跳转                                                                     |
| JGE  (label)    | 如果栈顶大于或等于零则跳转                                                               |
| JLE  (label)    | 如果栈顶小于或等于零则跳转                                                               |
| Call (procedure)| 调用一个过程，将栈偏移量设置为当前s栈长度                                             |
| Get  (usize)    | 获取栈的索引并将其复制到栈顶                                                               |
| Set  (usize)    | 将栈顶的值复制到索引                                                                   |
| GetArg  (usize) | 从调用栈的顶部获取第n个参数，用于过程                                                   |
| SetArg  (usize) | 设置调用栈顶部的第n个参数，用于过程                                                   |
| Noop             | 不做任何事情，由注释使用以保持指令指针与行对应                                             |
| Print            | 将栈顶的值作为整数打印出来                                                               |
| PrintC           | 将栈顶的值作为ASCII字符打印出来                                                         |
| PrintStack       | 打印整个栈，主要用于调试                                                                   |

你还可以使用行 `label $name` 设置标签，并通过使用 `Proc $name`、`Ret` 和 `End` 声明一个过程。有关更多详细信息，请参见 `test_files/procedure.bytecode` 或 `test_files/fib_recurse.bytecode`。

## 示例

在 `test_files/` 中：

`hello_world.bytecode` 打印 "Hello World"

`sum.bytecode` 打印从0到100所有整数的和

`fib.bytecode` 打印前40个斐波那契数

`fib_recurse.bytecode` 递归计算第35个斐波那契数

### 求和
```
Push 0
Push 0

label loop
-- [累加器，索引]
Get 0
Get 1
-- [累加器，索引，累加器，索引]
Add
-- [累加器，索引，累加器 + 索引]
Set 0
Pop
-- [累加器 + 索引，索引]

    -- [累加器，索引]
    Incr
    -- [累加器，索引 + 1]

    -- [累加器，索引]
    Get 1
    Push 100
    Sub
    -- [累加器，索引，索引 - 100]
    JNE loop
Pop

Get 0
Print
Push 10
PrintC
```
## 运行
```bash
cargo run --release sum.bytecode

# 4950
```
