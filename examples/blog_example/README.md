# README

本文主要介绍使用如果使用低代码工具来开发一个 blog 示例应用。

## Prerequisites

目前 dddappp 低代码工具以 Docker 镜像的方式发布，供开发者体验。

该工具所生成应用的链下服务使用 Java 语言编写，默认使用了 MySQL 数据库。但是本文不打算详细讲解链下服务的部署和测试，而是主要介绍如何使用 Rooch CLI 以及 jq 等命令行工具进行链上状态的查询以及对合约进行测试。

所以在开始体验前，你需要先：

* 安装 [Rooch CLI](https://github.com/rooch-network/rooch)。

* 安装 [Docker](https://docs.docker.com/engine/install/)。

* 安装 curl 以及 jp 命令（jp - commandline JSON processor）。我们在测试合约的时候可以使用 jp 来处理 JSON RPC 返回的 JSON 结果。

* （可选）安装 MySQL 数据库。可用于部署和测试链下服务。

* （可选）安装 JDK 和 Maven。用于构建和测试链下服务。

## 重现 blog example 的开发过程

你可以按照下面的介绍重现本示例应用的开发过程。

### 编写 DDDML 模型文件

我们的低代码工具依赖 DDDML（领域驱动设计建模语言）描述的领域模型来生成应用的各部分代码。

> **提示**
>
> 关于 DDDML，这里有一篇入门的介绍文章：[《DDDML 简介：开启去中心化应用低代码开发的钥匙》](https://github.com/wubuku/Dapp-LCDP-Demo/blob/main/IntroducingDDDML_CN.md)。这篇文章包含了本 Demo 使用的一些 DDDML 模型文件的详细讲解。

你可以创建一个目录，比如叫做 `test`，来放置应用的所有代码，然后在该目录下面创建一个子目录 `dddml`。我们一般在这个目录下放置按照 DDDML 的规范编写的模型文件。

在 dddml 目录下创建一个纯文本文件，命名为 `blog.yaml`，文件内容如下：

```yaml
aggregates:
  Article:
    metadata:
      Preprocessors: ["MOVE_CRUD_IT"]
    id:
      name: Id
      type: ObjectID

    properties:
      Title:
        type: String
        length: 200
      Body:
        type: String
        length: 2000
      Comments:
        itemType: Comment

    entities:
      Comment:
        metadata:
          Preprocessors: ["MOVE_CRUD_IT"]
        id:
          name: CommentSeqId
          type: u64
        properties:
          Commenter:
            type: String
            length: 100
          Body:
            type: String
            length: 500
```

上面的 DDDML 模型对于开发者而言其实十分浅白，但是我们下面还是会略作解释。

这些代码定义了一个名为 Article 的聚合及同名聚合根，以及一个名为 Comment 的聚合内部实体。 

#### “文章”聚合

在 `/aggregates/Article/metadata` 这个键结点下，我们定义了一些元数据，用来指示生成代码时应用的一些预处理器。这里我们使用了 `MOVE_CRUD_IT` 这个预处理器，它的作用是自动实体的 CRUD 操作逻辑。

在 `/aggregates/Article/id` 这个键结点下，我们定义了文章聚合根的 ID。文章的 ID 的名字为 `Id`，类型为 `ObjectID`。这里的 `ObjectID` 是一个平台特定的类型，我们假设现在正在开发一个基于 Rooch 的去中心化应用。

在 `/aggregates/Article/properties` 这个键结点下，我们定义了文章的属性分别表示文章的标题、正文和评论。

文章的标题（Title）属性是一个类型为 String 的属性，长度限制为 200 个字符。

文章的正文（Body）属性是一个类型为 String 的属性，长度限制为 2000 个字符。

文章的评论（Comments）属性是一个由类型是 `Comment` 的元素所组成的集合（`itemType: Comment`）。这里的 `Comment` 是一个聚合内部实体。

#### “评论”实体

在 `/aggregates/Article/entities/Comment` 这个键结点下，我们定义了“评论”这个聚合内部实体。

在这里定义的评论（聚合内部实体）的 `id` 是个 local ID（局部 ID），同样只要保证在同一篇文章内不同的评论之间这个 ID 的值具备唯一性就可以了。

我们将评论的 ID 命名为 `CommentSeqId`，声明其类型为 u64。

在 `/aggregates/Article/entities/Comment/metadata` 结点下我们也定义了一些元数据，同样使用了 `MOVE_CRUD_IT` 这个预处理器，让评论实体拥有自己的 CRUD 操作。

在 `/aggregates/Article/entities/Comment/properties` 结点下我们定义了评论的属性，分别表示评论者和评论内容。

评论者（Commenter）属性是一个类型为 String 的属性，长度限制为 100 个字符。

评论内容（Body）属性是一个类型为 String 的属性，长度限制为 500 个字符。

### 运行 dddappp 项目创建工具

使用 Docker 运行项目创建工具：

```shell
docker run \
-v /PATH/TO/test:/myapp \
wubuku/dddappp-rooch:0.0.1 \
--dddmlDirectoryPath /myapp/dddml \
--boundedContextName Test.RoochDemo \
--roochMoveProjectDirectoryPath /myapp/move \
--boundedContextRoochPackageName RoochDemo \
--boundedContextRoochNamedAddress rooch_demo \
--boundedContextJavaPackageName org.test.roochdemo \
--javaProjectsDirectoryPath /myapp/rooch-java-service \
--javaProjectNamePrefix roochdemo \
--pomGroupId test.roochdemo
```

上面的命令参数很直白：

* 注意将 `/PATH/TO/test` 替换为你实际放置应用代码的本机目录的路径。这一行表示将该本机目录挂载到容器内的 `/myapp` 目录。
* dddmlDirectoryPath 是 DDDML 模型文件所在的目录。它应该是容器内可以读取的目录路径。
* 把参数 boundedContextName 的值理解为你要开发的应用的名称即可。名称有多个部分时请使用点号分隔，每个部分使用 PascalCase 命名风格。Bounded-context 是领域驱动设计（DDD）中的一个术语，指的是一个特定的问题域范围，包含了特定的业务边界、约束和语言，这个概念你暂时不能理解也没有太大的关系。
* roochMoveProjectDirectoryPath 是放置链上 Rooch 合约代码的目录路径。它应该使用容器内可以读写的目录路径。
* boundedContextRoochPackageName 是链上 Rooch 合约的包名。建议采用 PascalCase 命名风格。
* boundedContextRoochNamedAddress 是链上 Rooch 合约默认的命名地址。建议采用 snake_case 命名风格。
* boundedContextJavaPackageName 是链下服务的 Java 包名。按照 Java 的命名规范，它应该全小写、各部分以点号分隔。
* javaProjectsDirectoryPath 是放置链下服务代码的目录路径。链下服务由多个模块（项目）组成。它应该使用容器内的可以读写的目录路径。
* javaProjectNamePrefix 是组成链下服务的各模块的名称前缀。建议使用一个全小写的名称。
* pomGroupId 链下服务的 GroupId，我们使用 Maven 作为链下服务的项目管理工具。它应该全小写、各部分以点号分隔。

上面的命令执行成功后，在本地目录 `/PATH/TO/test` 下应该会增加两个目录 `move` 以及 `rooch-java-service`。

此时你可以尝试编译链下服务。进入目录 `rooch-java-service`，执行：`mvn compile`。如果没有意外，编译应该可以成功。


## 测试应用

进入 `move` 目录，这里放置的是从模型生成的 Move 合约项目。执行 Move 编译命令：

```shell
rooch move build --named-addresses rooch_demo=0xf8e38d63a5208d499725e7ac4851c4a0836e45e2230041b7e3cf43e4738c47b4
```

如果没有意外，合约项目可以构建成功（输出的最后一行应该显示 `Success`），但是此时应该存在一些编译警告。那是因为一些以 `_logic.move` 结尾的 Move 源代码中引入（`use`）了一些没有用到的模块。

这些源代码文件是“业务逻辑”代码所在之处。如果你在 DDDML 文件中为聚合定义了一个方法（method），那么 dddappp 工具就会为你生成对应的一个名为 `{聚合名_方法名}_logic.move` 的 Move 代码文件，然后你需要在这个文件各种填充“业务逻辑”的实现代码。

不过，上面我们使用的 `MOVE_CRUD_IT` 预处理器更近一步，直接为我们生成简单的 CRUD 方法的默认实现。当然，我们可以检查一下这些“填充好的默认逻辑”，视自己的需要修改它们。

已经存在的“业务逻辑”代码文件是（可以执行命令 `ls sources/*_logic.move` 列出）：

```shell
sources/article_add_comment_logic.move          sources/article_delete_logic.move               sources/article_update_comment_logic.move
sources/article_create_logic.move               sources/article_remove_comment_logic.move       sources/article_update_logic.move
```

打开它们，移除那些多余的 `use` 语句。如果你的 IDE 安装了一些 Move 语言的插件，可能你只需要使用“格式化”功能对这几个源文件重新格式化一下即可。

---

【TBD】

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




