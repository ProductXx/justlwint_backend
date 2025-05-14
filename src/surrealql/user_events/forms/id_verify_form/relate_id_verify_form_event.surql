DEFINE EVENT IF NOT EXISTS relate_id_verify_forms ON TABLE tb_id_verify_forms WHEN $event = "CREATE" THEN {
    LET $user_id = $after.user_id;
    LET $form_id = $after.id;
    RELATE $user_id -> tb_form_upload -> $form_id SET date = time::now();
};
