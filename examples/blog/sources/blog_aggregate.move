// <autogenerated>
//   This file was generated by dddappp code generator.
//   Any changes made to this file manually will be lost next time the file is regenerated.
// </autogenerated>

module rooch_examples::blog_aggregate {
    use moveos_std::object_id::ObjectID;
    use moveos_std::storage_context::StorageContext;
    use rooch_examples::blog;
    use rooch_examples::blog_create_logic;
    use rooch_examples::blog_delete_logic;
    use rooch_examples::blog_update_logic;
    use std::string::String;

    public entry fun create(
        storage_ctx: &mut StorageContext,
        account: &signer,
        name: String,
        articles: vector<ObjectID>,
    ) {
        let blog_created = blog_create_logic::verify(
            storage_ctx,
            account,
            name,
            articles,
        );
        let blog = blog_create_logic::mutate(
            storage_ctx,
            account,
            &blog_created,
        );
        blog::add_blog(storage_ctx, account, blog);
        blog::emit_blog_created(storage_ctx, blog_created);
    }


    public entry fun update(
        storage_ctx: &mut StorageContext,
        account: &signer,
        name: String,
        articles: vector<ObjectID>,
    ) {
        let blog = blog::remove_blog(storage_ctx);
        let blog_updated = blog_update_logic::verify(
            storage_ctx,
            account,
            name,
            articles,
            &blog,
        );
        let updated_blog = blog_update_logic::mutate(
            storage_ctx,
            account,
            &blog_updated,
            blog,
        );
        blog::update_version_and_add(storage_ctx, account, updated_blog);
        blog::emit_blog_updated(storage_ctx, blog_updated);
    }


    public entry fun delete(
        storage_ctx: &mut StorageContext,
        account: &signer,
    ) {
        let blog = blog::remove_blog(storage_ctx);
        let blog_deleted = blog_delete_logic::verify(
            storage_ctx,
            account,
            &blog,
        );
        let updated_blog = blog_delete_logic::mutate(
            storage_ctx,
            account,
            &blog_deleted,
            blog,
        );
        blog::drop_blog(updated_blog);
        blog::emit_blog_deleted(storage_ctx, blog_deleted);
    }

}
