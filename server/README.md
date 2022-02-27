# 概述

这是一个`vs code extension` 的服务端。

使用了百度通用翻译 api, 当前为免费使用, 限制 1 秒 1 次请求
请自行申请百度翻译帐号, 在 `server/` 目录下自建`.env`文件:

```bash
baidu_appid = your baidu appid
baidu_secret = your baidu secret
baidu_salt = any string you want
```
