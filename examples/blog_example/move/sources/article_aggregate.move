// <autogenerated>
//   This file was generated by dddappp code generator.
//   Any changes made to this file manually will be lost next time the file is regenerated.
// </autogenerated>

module rooch_demo::article_aggregate {
    use moveos_std::object_id::ObjectID;
    use moveos_std::storage_context::StorageContext;
    use rooch_demo::article;
    use rooch_demo::article_create_logic;
    use rooch_demo::article_delete_logic;
    use rooch_demo::article_update_logic;
    use std::string::String;

    public entry fun create(
        storage_ctx: &mut StorageContext,
        account: &signer,
        title: String,
        body: String,
    ) {
        let article_created = article_create_logic::verify(
            storage_ctx,
            account,
            title,
            body,
        );
        let article_obj = article_create_logic::mutate(
            storage_ctx,
            &article_created,
        );
        article::set_article_created_id(&mut article_created, article::id(&article_obj));
        article::add_article(storage_ctx, article_obj);
        article::emit_article_created(storage_ctx, article_created);
    }


    public entry fun update(
        storage_ctx: &mut StorageContext,
        account: &signer,
        id: ObjectID,
        title: String,
        body: String,
    ) {
        let article_obj = article::remove_article(storage_ctx, id);
        let article_updated = article_update_logic::verify(
            storage_ctx,
            account,
            title,
            body,
            &article_obj,
        );
        let updated_article_obj = article_update_logic::mutate(
            storage_ctx,
            &article_updated,
            article_obj,
        );
        article::update_version_and_add(storage_ctx, updated_article_obj);
        article::emit_article_updated(storage_ctx, article_updated);
    }


    public entry fun delete(
        storage_ctx: &mut StorageContext,
        account: &signer,
        id: ObjectID,
    ) {
        let article_obj = article::remove_article(storage_ctx, id);
        let article_deleted = article_delete_logic::verify(
            storage_ctx,
            account,
            &article_obj,
        );
        let updated_article_obj = article_delete_logic::mutate(
            storage_ctx,
            &article_deleted,
            article_obj,
        );
        article::update_version_and_add(storage_ctx, updated_article_obj);
        article::emit_article_deleted(storage_ctx, article_deleted);
    }

}
