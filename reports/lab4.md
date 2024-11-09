### 简单总结你实现的功能:
- 这次就是实现三个系统调用, link, unlink和fstat.

- link和unlink调用中, 我都将传进来的指针转换为字符串后丢给`ROOT_INDOE`进行再具体操作, (在link中如过两个字符串相等就返回-1). 关于具体的链接或者解链接, 我在`Struct Inode`添加link和unlink函数, 这两个方法也是只有根目录的Inode会调用.

- 在link中, 先利用`read_disk_inode`和`find_inode_id`先找到old_name文件的old_inode_id, 再用`modify_disk_inode`向disk_inode中新加上目录项(new_name, old_inode_id), 再用文件系统的`get_disk_inode_pos`方法通过old_inode_id找到被链接文件的inode, 再让这个inode调用一个改变记录链接数量的方法将该文件的链接数加1. 
<br>(我将`INODE_DIRECT_COUNT`减去1, 加一个nlink_num存在了disk_inode里记录链接数, 这个方法用`modify_disk_inode`将inode对的disk_inode里的nlink_num进行修改) 

- unlink跟link也差不多, 找到inode等东西后做一些不同的操作即可.

- 关于fstat, 系统调用只传进来一个fd, 那用fd索引fd_table, 只能得到一个File特性. 于是我给File特性增加一种方法, 直接返回stat结构体, 这个方法对标准输入输出无效, 只有OSInode实现. 接着就是做些工作, 将Stat结构体所需要的信息穿过层层抽象, 由DiskInode到Inode到OSInode, 填进Stat即可. (为了方便, 我把inode_id跟Inode绑在一起了, 每个Inode都带着id)

### 问答题
- ch6
  - root_inode就是我们文件系统的根目录, 由于这个文件系统只有一个目录, 它记录着系统里的所有文件. 如果它损坏了, 那整个文件系统就失效了.

- ch7
  - 比如说平时想在文件中找到某种pattern:
  ```bash
  seven@localhost ~/L/2/reports (ch6)> cat lab4.md | grep "pattern"
    - 比如说平时想在文件中找到某种pattern:
  seven@localhost ~/L/2/reports (ch6)> 
  ```

  - 可以使用消息队列.
    - 所有进程都使用一个消息队列, 往其中读写消息. 任意进程发送消息, 直接往队列, 每个消息对需包含接收方的id. 任意进程读取消息, 只有消息id与自己匹配才读取.

### 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

        无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

        无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

### Option

