// <autogenerated>
//   This file was generated by dddappp code generator.
//   Any changes made to this file manually will be lost next time the file is regenerated.
// </autogenerated>

module rooch_demo::article {
    use moveos_std::events;
    use moveos_std::object::{Self, Object};
    use moveos_std::object_id::ObjectID;
    use moveos_std::object_storage;
    use moveos_std::storage_context::{Self, StorageContext};
    use moveos_std::table::{Self, Table};
    use moveos_std::tx_context;
    use rooch_demo::comment::{Self, Comment};
    use std::error;
    use std::option;
    use std::signer;
    use std::string::String;
    friend rooch_demo::article_create_logic;
    friend rooch_demo::article_update_logic;
    friend rooch_demo::article_delete_logic;
    friend rooch_demo::article_aggregate;

    const EID_DATA_TOO_LONG: u64 = 102;
    const EINAPPROPRIATE_VERSION: u64 = 103;
    const ENOT_GENESIS_ACCOUNT: u64 = 105;

    struct CommentTableItemAdded has key {
        article_id: ObjectID,
        comment_seq_id: u64,
    }

    public fun initialize(storage_ctx: &mut StorageContext, account: &signer) {
        assert!(signer::address_of(account) == @rooch_demo, error::invalid_argument(ENOT_GENESIS_ACCOUNT));
        let _ = storage_ctx;
        let _ = account;
    }

    struct Article has key {
        version: u64,
        title: String,
        body: String,
        comments: Table<u64, Comment>,
    }

    /// get object id
    public fun id(article_obj: &Object<Article>): ObjectID {
        object::id(article_obj)
    }

    public fun version(article_obj: &Object<Article>): u64 {
        object::borrow(article_obj).version
    }

    public fun title(article_obj: &Object<Article>): String {
        object::borrow(article_obj).title
    }

    public(friend) fun set_title(article_obj: &mut Object<Article>, title: String) {
        object::borrow_mut(article_obj).title = title;
    }

    public fun body(article_obj: &Object<Article>): String {
        object::borrow(article_obj).body
    }

    public(friend) fun set_body(article_obj: &mut Object<Article>, body: String) {
        object::borrow_mut(article_obj).body = body;
    }

    public(friend) fun add_comment(storage_ctx: &mut StorageContext, article_obj: &mut Object<Article>, comment: Comment) {
        let comment_seq_id = comment::comment_seq_id(&comment);
        table::add(&mut object::borrow_mut(article_obj).comments, comment_seq_id, comment);
        events::emit_event(storage_ctx, CommentTableItemAdded {
            article_id: id(article_obj),
            comment_seq_id,
        });
    }

    public(friend) fun remove_comment(article_obj: &mut Object<Article>, comment_seq_id: u64) {
        let comment = table::remove(&mut object::borrow_mut(article_obj).comments, comment_seq_id);
        comment::drop_comment(comment);
    }

    public(friend) fun borrow_mut_comment(article_obj: &mut Object<Article>, comment_seq_id: u64): &mut Comment {
        table::borrow_mut(&mut object::borrow_mut(article_obj).comments, comment_seq_id)
    }

    public fun borrow_comment(article_obj: &Object<Article>, comment_seq_id: u64): &Comment {
        table::borrow(&object::borrow(article_obj).comments, comment_seq_id)
    }

    public fun comments_contains(article_obj: &Object<Article>, comment_seq_id: u64): bool {
        table::contains(&object::borrow(article_obj).comments, comment_seq_id)
    }

    fun new_article(
        tx_ctx: &mut tx_context::TxContext,
        title: String,
        body: String,
    ): Article {
        assert!(std::string::length(&title) <= 200, EID_DATA_TOO_LONG);
        assert!(std::string::length(&body) <= 2000, EID_DATA_TOO_LONG);
        Article {
            version: 0,
            title,
            body,
            comments: table::new<u64, Comment>(tx_ctx),
        }
    }

    struct ArticleCreated has key {
        id: option::Option<ObjectID>,
        title: String,
        body: String,
    }

    public fun article_created_id(article_created: &ArticleCreated): option::Option<ObjectID> {
        article_created.id
    }

    public(friend) fun set_article_created_id(article_created: &mut ArticleCreated, id: ObjectID) {
        article_created.id = option::some(id);
    }

    public fun article_created_title(article_created: &ArticleCreated): String {
        article_created.title
    }

    public fun article_created_body(article_created: &ArticleCreated): String {
        article_created.body
    }

    public(friend) fun new_article_created(
        title: String,
        body: String,
    ): ArticleCreated {
        ArticleCreated {
            id: option::none(),
            title,
            body,
        }
    }

    struct ArticleUpdated has key {
        id: ObjectID,
        version: u64,
        title: String,
        body: String,
    }

    public fun article_updated_id(article_updated: &ArticleUpdated): ObjectID {
        article_updated.id
    }

    public fun article_updated_title(article_updated: &ArticleUpdated): String {
        article_updated.title
    }

    public fun article_updated_body(article_updated: &ArticleUpdated): String {
        article_updated.body
    }

    public(friend) fun new_article_updated(
        article_obj: &Object<Article>,
        title: String,
        body: String,
    ): ArticleUpdated {
        ArticleUpdated {
            id: id(article_obj),
            version: version(article_obj),
            title,
            body,
        }
    }

    struct ArticleDeleted has key {
        id: ObjectID,
        version: u64,
    }

    public fun article_deleted_id(article_deleted: &ArticleDeleted): ObjectID {
        article_deleted.id
    }

    public(friend) fun new_article_deleted(
        article_obj: &Object<Article>,
    ): ArticleDeleted {
        ArticleDeleted {
            id: id(article_obj),
            version: version(article_obj),
        }
    }


    public(friend) fun create_article(
        storage_ctx: &mut StorageContext,
        title: String,
        body: String,
    ): Object<Article> {
        let tx_ctx = storage_context::tx_context_mut(storage_ctx);
        let article = new_article(
            tx_ctx,
            title,
            body,
        );
        let obj_owner = tx_context::sender(tx_ctx);
        let article_obj = object::new(
            tx_ctx,
            obj_owner,
            article,
        );
        article_obj
    }

    public(friend) fun update_version_and_add(storage_ctx: &mut StorageContext, article_obj: Object<Article>) {
        object::borrow_mut(&mut article_obj).version = object::borrow( &mut article_obj).version + 1;
        //assert!(object::borrow(&article_obj).version != 0, EINAPPROPRIATE_VERSION);
        private_add_article(storage_ctx, article_obj);
    }

    public(friend) fun remove_article(storage_ctx: &mut StorageContext, obj_id: ObjectID): Object<Article> {
        let obj_store = storage_context::object_storage_mut(storage_ctx);
        object_storage::remove<Article>(obj_store, obj_id)
    }

    public(friend) fun add_article(storage_ctx: &mut StorageContext, article_obj: Object<Article>) {
        assert!(object::borrow(&article_obj).version == 0, EINAPPROPRIATE_VERSION);
        private_add_article(storage_ctx, article_obj);
    }

    fun private_add_article(storage_ctx: &mut StorageContext, article_obj: Object<Article>) {
        assert!(std::string::length(&object::borrow(&article_obj).title) <= 200, EID_DATA_TOO_LONG);
        assert!(std::string::length(&object::borrow(&article_obj).body) <= 2000, EID_DATA_TOO_LONG);
        let obj_store = storage_context::object_storage_mut(storage_ctx);
        object_storage::add(obj_store, article_obj);
    }

    public fun get_article(storage_ctx: &mut StorageContext, obj_id: ObjectID): Object<Article> {
        remove_article(storage_ctx, obj_id)
    }

    public fun return_article(storage_ctx: &mut StorageContext, article_obj: Object<Article>) {
        private_add_article(storage_ctx, article_obj);
    }

    public(friend) fun drop_article(article_obj: Object<Article>) {
        let (_id, _owner, article) =  object::unpack(article_obj);
        let Article {
            version: _version,
            title: _title,
            body: _body,
            comments,
        } = article;
        table::destroy_empty(comments);
    }

    public(friend) fun emit_article_created(storage_ctx: &mut StorageContext, article_created: ArticleCreated) {
        events::emit_event(storage_ctx, article_created);
    }

    public(friend) fun emit_article_updated(storage_ctx: &mut StorageContext, article_updated: ArticleUpdated) {
        events::emit_event(storage_ctx, article_updated);
    }

    public(friend) fun emit_article_deleted(storage_ctx: &mut StorageContext, article_deleted: ArticleDeleted) {
        events::emit_event(storage_ctx, article_deleted);
    }

}
