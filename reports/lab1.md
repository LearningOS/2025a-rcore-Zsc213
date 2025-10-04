#### 完成的功能：

实现sys_trace, 功能0直接将_id转化为*const u8并返回；

功能1将_id转化为*mut u8的可变指针从而将data写入该地址处；

功能2的实现需要在task manager中存储各任务信息的TaskControlBlock结构体中增加记录系统调用次数的数组，并在task程序进行系统调用时于trap_handler中将调用次数记录在对应的结构中，sys_trace直接获得记录的调用次数。记录的函数在进入sys_call之前所以本次调用也会记录.

#### 简答：

1. user程序尝试执行特权指令会导致内核panic

   访问非法地址: PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it. 执行特权指令: IllegalInstruction in application, kernel killed it. 访问特殊寄存器: IllegalInstruction in application, kernel killed it.

2. (1) 进入__restore时sp是程序内核栈的栈指针。restore会在第一次进入用户态和user程序的的系统调用返回回到用户态触发。第一次进入用户态内核会构造特殊上下文，在运行第一个user程序前通过restore从内核态进入用户态；user程序在系统调用trap_handler执行结束后由restore进行上下文恢复（对应触发系统调用的alltraps保存上下文）再返回用户态。

   (2) t0,t1,t2分别用于保存并恢复sstatus程序状态、spec系统调用时程序的pc指针、sscratch程序的用户栈。

   (3) 进入内核态保存点 上下文中x2传递给trap_handler的参数，恢复上下文时不需要使用；x4由于没有开启线程也不不需要使用。

   (4) 该指令在系统调用结束后返回用户态时执行，sp指向程序的用户栈，sscratch指向程序的内核栈。

   (5) sret后发生状态切换。由处理器自动执行状态切换，从spec中恢复pc等。

   (6) 指令用于系统调用进入内核态后将程序的sscratch写入sp，sp指向内核栈，sscratch指向用户栈。

   (7) 用户程序使用ecall指令触发系统调用由用户态进入内核态。

#### 荣誉准则：

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > 仅本人

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > rcore-tutorial-book-v3: http://rcore-os.cn/rCore-Tutorial-Book-v3/index.html
   >
   > rcore-tutorial-guide-2025S: https://learningos.cn/rCore-Tutorial-Guide-2025S/honorcode.html

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。



