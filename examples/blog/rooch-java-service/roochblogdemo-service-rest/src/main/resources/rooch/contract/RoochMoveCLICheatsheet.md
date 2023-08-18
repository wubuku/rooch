# Rooch Move CLI Cheatsheet

[ToC]

## Article aggregate

### UpdateComment method

```shell
rooch move run --sender-account _SENDER_ADDRESS_ --function '_CONTRACT_ADDRESS_::article_aggregate::update_comment' \
--args 'object_id:id' u64:comment_seq_id 'string:commenter' 'string:body' address:owner
```

### RemoveComment method

```shell
rooch move run --sender-account _SENDER_ADDRESS_ --function '_CONTRACT_ADDRESS_::article_aggregate::remove_comment' \
--args 'object_id:id' u64:comment_seq_id
```

### AddComment method

```shell
rooch move run --sender-account _SENDER_ADDRESS_ --function '_CONTRACT_ADDRESS_::article_aggregate::add_comment' \
--args 'object_id:id' 'string:commenter' 'string:body'
```

### Create method

```shell
rooch move run --sender-account _SENDER_ADDRESS_ --function '_CONTRACT_ADDRESS_::article_aggregate::create' \
--args 'string:title' 'string:body'
```

### Update method

```shell
rooch move run --sender-account _SENDER_ADDRESS_ --function '_CONTRACT_ADDRESS_::article_aggregate::update' \
--args 'object_id:id' 'string:title' 'string:body'
```

### Delete method

```shell
rooch move run --sender-account _SENDER_ADDRESS_ --function '_CONTRACT_ADDRESS_::article_aggregate::delete' \
--args 'object_id:id'
```

