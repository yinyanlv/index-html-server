# index-html-server
Server for index.html。在不需要安装其它依赖的情况下，为React、Angular、Vue编译后生成的静态文件，提供一个静态网站服务器。

## 使用方法
根据相应的操作系统，下载本项目对应的可执行文件到目标文件夹，然后，执行该可执行文件，就可以成功启动静态网站服务器。

## 配置项
| 属性 | 说明 |
| - | - |
| folderPath | 静态文件夹路径（默认：./static/）
| entryFile | 网站入口文件（默认：index.html）
| port | 服务器端口号（默认：8000）

## 自定义配置
创建并编辑config.json文件（相关配置项如上），然后，将该配置文件放置到本项目可执行文件所在的文件夹下。

**通过命令行，可自定义配置文件的名称和路径**，用法如下：
```
index-html-server --config ../../abc/custome_config.json
或
index-html-server -c ../../abc/custome_config.json
```
**备注：**可将本项目的可执行文件添加到系统环境变量中，这样就可以在命令行中直接使用index-html-server。


