# pve_power
pve节能、自动关机挂起

#pve虚拟机节能工具，经常在虚拟机办公，忘记关闭宿主机，导致硬件消耗和节能问题，所以写了个小工具
1、遍历所有虚拟机、如果处于运行状态会自动挂起
2、挂起虚拟机100秒后对宿主机关机
3、可以使用stop 或者 start 停止该工具
4、可以使用 时间参数 暂时停止该工具 例如:mypve 3 //暂停三天

添加任务计划得root,拷贝本文件到bin目录
vim /etc/crontab

30 23 * * * root /bin/mypve

#example:

mypve start | stop

mypve 3
