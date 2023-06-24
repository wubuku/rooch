module rooch_demo::article_delete_logic {
    use moveos_std::object::Object;
    use moveos_std::storage_context::StorageContext;
    use rooch_demo::article;

    friend rooch_demo::article_aggregate;

    public(friend) fun verify(
        storage_ctx: &mut StorageContext,
        account: &signer,
        article_obj: &Object<article::Article>,
    ): article::ArticleDeleted {
        let _ = storage_ctx;
        let _ = account;
        article::new_article_deleted(article_obj)
    }

    public(friend) fun mutate(
        storage_ctx: &mut StorageContext,
        article_deleted: &article::ArticleDeleted,
        article_obj: Object<article::Article>,
    ): Object<article::Article> {
        let id = article::id(&article_obj);
        let _ = storage_ctx;
        let _ = article_deleted;
        let _ = id; //todo support delete article object
        article_obj
    }
}
