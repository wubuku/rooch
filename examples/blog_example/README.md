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

再获取另外一篇文章的 ObjectID（注意 `jq` 的路径参数，获取的是“第二个” ArticleCreated 事件的信息）：

```shell
curl --location --request POST 'http://localhost:50051' \
--header 'Content-Type: application/json' \
--data-raw '{
 "id":101,
 "jsonrpc":"2.0",
 "method":"rooch_getEventsByEventHandle",
 "params":["0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4::article::ArticleCreated", null, 1000]
}' | jq '.result.data[1].parsed_event_data.value.id.value.vec[0]'
```

假设，得到的文章的 ObjectID 是 `0x9ab4207df54d07223f294cabd08b5c1cbcc1e262086685fcfb5a540cf62e2dae`。

我们可以给这篇文章添加一个评论：

```shell
rooch move run --function 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4::article_aggregate::add_comment --sender-account 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4 --args 'object_id:0x9ab4207df54d07223f294cabd08b5c1cbcc1e262086685fcfb5a540cf62e2dae' 'u64:1' 'string:Anonymous' 'string:"A test comment"'
```

我们可以给这篇文章多添加几条评论，比如：

```shell
rooch move run --function 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4::article_aggregate::add_comment --sender-account 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4 --args 'object_id:0x9ab4207df54d07223f294cabd08b5c1cbcc1e262086685fcfb5a540cf62e2dae' 'u64:2' 'string:Anonymous2' 'string:"A test comment2"'
```

通过查询事件，我们知道这篇文章都有那些评论：

```shell
curl --location --request POST 'http://localhost:50051' \
--header 'Content-Type: application/json' \
--data-raw '{
 "id":101,
 "jsonrpc":"2.0",
 "method":"rooch_getEventsByEventHandle",
 "params":["0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4::article::CommentTableItemAdded", null, 10000]
}' | jq '.result.data[] | select(.parsed_event_data.value.article_id == "0x9ab4207df54d07223f294cabd08b5c1cbcc1e262086685fcfb5a540cf62e2dae")'
```

在我们的 Move 合约中，一篇文章的所有评论，是保存在嵌入在该文章对象的一个 table 中的。

我们可以通过 JSON RPC 查询评论的具体信息。

首先，我们要取得文章的评论表（comment table）的 handle：

```shell
curl --location --request POST 'http://127.0.0.1:50051/' \
--header 'Content-Type: application/json' \
--data-raw '{
 "id":101,
 "jsonrpc":"2.0",
 "method":"rooch_getAnnotatedStates",
 "params":["/object/0x9ab4207df54d07223f294cabd08b5c1cbcc1e262086685fcfb5a540cf62e2dae"]
}' | jq '.result[0].move_value.value.value.value.comments.value.handle'
```

假设，得到的 table handle 为 `0xad1a904b42a70fb0a64b545b4b14e7caef81dc40b7bd719aeb663f0db24dc57e`。

那么，我们可以通过下面的方式获取的评论的具体信息（get table item by handle and key）：

```shell
curl --location --request POST 'http://127.0.0.1:50051/' \
--header 'Content-Type: application/json' \
--data-raw '{
 "id":101,
 "jsonrpc":"2.0",
 "method":"rooch_getAnnotatedStates",
 "params":["/table/0xad1a904b42a70fb0a64b545b4b14e7caef81dc40b7bd719aeb663f0db24dc57e/0x0100000000000000"]
}'
```

注意上面的命令，路径中的 table key 的值。比如，类型为 u64 的整数值 1 的 BCS 序列化结果，以十六进制字符串表示为 `0x0100000000000000`。

移除评论：

```shell
rooch move run --function 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4::article_aggregate::remove_comment --sender-account 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4 --args 'object_id:0x9ab4207df54d07223f294cabd08b5c1cbcc1e262086685fcfb5a540cf62e2dae' 'u64:1'
```

再次执行上面的 curl 命令查询评论，这次会返回类似这样的信息：

```json
{"jsonrpc":"2.0","result":[null],"id":101}
```

~~因为我们后面这篇文章还有未被删除的评论，所以如果现在想要删除它，应该不会成功。尝试执行~~：

```shell
rooch move run --function 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4::article_aggregate::delete --sender-account 0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4 --args 'object_id:0x9ab4207df54d07223f294cabd08b5c1cbcc1e262086685fcfb5a540cf62e2dae'
```




