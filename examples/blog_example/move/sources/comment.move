// <autogenerated>
//   This file was generated by dddappp code generator.
//   Any changes made to this file manually will be lost next time the file is regenerated.
// </autogenerated>

module rooch_demo::comment {
    use std::string::String;
    friend rooch_demo::article_create_logic;
    friend rooch_demo::article_update_logic;
    friend rooch_demo::article_delete_logic;
    friend rooch_demo::article_add_comment_logic;
    friend rooch_demo::article_remove_comment_logic;
    friend rooch_demo::article;

    const EID_DATA_TOO_LONG: u64 = 102;

    struct Comment has store {
        comment_seq_id: u64,
        commenter: String,
        body: String,
    }

    public fun comment_seq_id(comment: &Comment): u64 {
        comment.comment_seq_id
    }

    public fun commenter(comment: &Comment): String {
        comment.commenter
    }

    public(friend) fun set_commenter(comment: &mut Comment, commenter: String) {
        comment.commenter = commenter;
    }

    public fun body(comment: &Comment): String {
        comment.body
    }

    public(friend) fun set_body(comment: &mut Comment, body: String) {
        comment.body = body;
    }

    public(friend) fun new_comment(
        comment_seq_id: u64,
        commenter: String,
        body: String,
    ): Comment {
        assert!(std::string::length(&commenter) <= 100, EID_DATA_TOO_LONG);
        assert!(std::string::length(&body) <= 500, EID_DATA_TOO_LONG);
        Comment {
            comment_seq_id,
            commenter,
            body,
        }
    }

    public(friend) fun drop_comment(comment: Comment) {
        let Comment {
            comment_seq_id: _,
            commenter: _,
            body: _,
        } = comment;
    }


}
