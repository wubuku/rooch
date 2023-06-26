# README

先运行本地服务器。

```shell
rooch server start
```

发布合约：

```shell
rooch move publish --named-addresses rooch_demo=0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4
```

当你看到类似这样的输出（`status` 为 `executed`），就可以确认发布操作已经成功执行了：

```shell
{
  //...
  "execution_info": {
    //...
    "status": {
      "type": "executed"
    }
  },
  //...
}
```

初始化合约：

```shell
rooch move run --function 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4::rooch_demo_init::initialize --sender-account 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4
```

创建 Article：

```shell
rooch move run --function 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4::article_aggregate::create --sender-account 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4 --args 'string:Hello' 'string:World!'
```

可以更换一下第一个参数（`title`）和第二个参数（`body`）的内容，多创建几篇文章。

可以通过查询事件，得到已创建的 Article 的 ObjectID：

```shell
curl --location --request POST 'http://localhost:50051' \
--header 'Content-Type: application/json' \
--data-raw '{
 "id":101,
 "jsonrpc":"2.0",
 "method":"rooch_getEventsByEventHandle",
 "params":["0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4::article::ArticleCreated", null, 1000]
}'
```

> **提示**
> 
> 在使用 `jp` 命令（jq - commandline JSON processor）之前，你可能需要在本机上先安装它。

可以在上面的命令最尾添加一个管道操作（` | jq '.result.data[0].parsed_event_data.value.id.value.vec[0]'`），来快速筛选出第一篇文章的 ObjectID。 那么，命令像这样：

```shell
curl --location --request POST 'http://localhost:50051' \
--header 'Content-Type: application/json' \
--data-raw '{
 "id":101,
 "jsonrpc":"2.0",
 "method":"rooch_getEventsByEventHandle",
 "params":["0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4::article::ArticleCreated", null, 1000]
}' | jq '.result.data[0].parsed_event_data.value.id.value.vec[0]'
```

使用 Rooch CLI 来查询对象的状态（假设得到的文章的 ObjectID 为 `0xd2443e42454e8705135ca38c094fe524da6e0de0e8862b8073d4039acaf11995`）：

```shell
rooch object --id 0xd2443e42454e8705135ca38c094fe524da6e0de0e8862b8073d4039acaf11995
```

更新 Article：

```shell
rooch move run --function 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4::article_aggregate::update --sender-account 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4 --args 'object_id:0xd2443e42454e8705135ca38c094fe524da6e0de0e8862b8073d4039acaf11995' 'string:Foo' 'string:Bar'
```

除了使用 Rooch CLI，你还可以通过调用 JSON RPC 来查询对象的状态：

```shell
curl --location --request POST 'http://127.0.0.1:50051/' \
--header 'Content-Type: application/json' \
--data-raw '{
 "id":101,
 "jsonrpc":"2.0",
 "method":"rooch_getAnnotatedStates",
 "params":["/object/0xd2443e42454e8705135ca38c094fe524da6e0de0e8862b8073d4039acaf11995"]
}'
```

删除 Article：

```shell
rooch move run --function 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4::article_aggregate::delete --sender-account 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4 --args 'object_id:0xd2443e42454e8705135ca38c094fe524da6e0de0e8862b8073d4039acaf11995'
```

再获取一篇文章的 ObjectID（比如 `0xd2443e42454e8705135ca38c094fe524da6e0de0e8862b8073d4039acaf11995`） ：

```shell
curl --location --request POST 'http://localhost:50051' \
--header 'Content-Type: application/json' \
--data-raw '{
 "id":101,
 "jsonrpc":"2.0",
 "method":"rooch_getEventsByEventHandle",
 "params":["0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4::article::ArticleCreated", null, 1000]
}' | jq '.result.data[0].parsed_event_data.value.id.value.vec[0]'
```

给这篇文章添加一个评论：

```shell
rooch move run --function 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4::article_aggregate::add_comment --sender-account 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4 --args 'object_id:0xd2443e42454e8705135ca38c094fe524da6e0de0e8862b8073d4039acaf11995' 'u64:1' 'string:Anonymous' 'string:"A test comment"'
```

