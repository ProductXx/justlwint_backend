DEFINE EVENT IF NOT EXISTS post_upload ON TABLE tb_posts WHEN $event = "CREATE" THEN {
    LET $author_id = $after.author_id;
    LET $post_id = $after.id;

    LET $noti_content = {
        date: time::now(),
        noti_title: "Post uploaded, your post is now available.",
        noti_type: 'PostUploaded',
        noti_message: Null,
        noti_for_user: $author_id,
        actions: Null
    };

    CREATE tb_notifications CONTENT $noti_content;
    
    RELATE $author_id -> tb_upload_posts -> $post_id SET date = time::now();
};
