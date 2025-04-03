# ch5报告
## 实验要求
    想要实现spawn只需要参考fork和exec和new的实现，要实现spawn只需要把fork和exec结合起来，如下：
    1、获取当前进程任务控制块
    2、从参数中获取需要的app的地址空间以及数据。
    3、获取其trap物理页
    4、申请一个新的pid
    5、用新获得的资源新建任务控制块。
    6、把新任务块加入到就绪队列中。
    7、初始化trap上下文

    stride 调度算法
    1、在内存控制块inner中加入pass和stride两个变量，定义全局常量BIG_STRIDE
    2、发生优先级修改系统调用时,计算新的pass值
    3、修改任务控制块中的VecQueue从队列转为Vec，并加入stride值，每次插入时同时插入stride值，当需要获取下一个任务时，寻找stride值最小的任务的序号，从Vec中取出返回即可。
    4、每次运行任务时更新stride值，stride += pass

### 问答作业
#### stride 算法原理非常简单，但是有一个比较大的问题。例如两个 pass = 10 的进程，使用 8bit 无符号整形储存 stride， p1.stride = 255, p2.stride = 250，在 p2 执行一个时间片后，理论上下一次应该 p1 执行。

    问题：实际情况是轮到 p1 执行吗？为什么？
**回答**：
    因为是8bit，因此250+10 = 260 > 255，因此p1的stride值会溢出，因此会变成4，因此还是执行p2。

#### 我们之前要求进程优先级 >= 2 其实就是为了解决这个问题。可以证明， 在不考虑溢出的情况下 , 在进程优先级全部 >= 2 的情况下，如果严格按照算法执行，那么 STRIDE_MAX – STRIDE_MIN <= BigStride / 2。

问题：为什么？尝试简单说明（不要求严格证明）。
**回答**：
    假设当前时刻刚好STRIDE_MAX = stride + BigStride / priority，
    又因为在上一时刻stride是最小的值（只有最小的才会被选中），因此该stride < 任意其他stride，
    因此STRIDE_MAX - 任意其他stride = -任意数 + BigStride / priority <= BigStride / priority，
    又因为priority >= 2，
    因此STRIDE_MAX - STRIDE_MIN <= BigStride / 2。

问题：已知以上结论，考虑溢出的情况下，可以为 Stride 设计特别的比较器，让 BinaryHeap<Stride> 的 pop 方法能返回真正最小的 Stride。补全下列代码中的 partial_cmp 函数，假设两个 Stride 永远不会相等。
**回答**:
    根据以上结论，如果发生溢出，则原来本该最大的stride会因为溢出而变得很小，原来的加BigStride / 2会因为溢出而少去1。
    因此STRIDE_MAX - STRIDE_MIN > BigStride / 2,就要按溢出后的规则处理。
    pub struct Stride(pub usize);

    impl PartialOrd for Stride {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.0.abs_diff(other.0) <= HALF_BIG_STRIDE.0{//无溢出
                self.0.partial_cmp(&other.0)
            }else{
                other.0.partial_cmp(&self.0)
            }
        }
    }

    impl PartialEq for Stride {
        fn eq(&self, _other: &Self) -> bool {
            false
        }
    }



## 荣誉准则
### 1.在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

跟Deepseek做交流，提问了问答作业的原理，代码自己写的。

### 2.此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

无

### 3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

### 4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。



