DEFINE EVENT IF NOT EXISTS id_verify_form_status ON TABLE tb_id_verify_forms WHEN $event = "UPDATE" THEN {
    LET $user_id = $after.user_id;
    LET $form_id = $after.id;
    LET $noti_message = $after.message;
    IF $after.status_type IS "Rejected" THEN {
        LET $noti_content = {
            date: time::now(),
            noti_title: "Your Id form has been rejected",
            noti_message: $noti_message,
            noti_type: "IdVerifyFormDenied",
            noti_for_user: $user_id,
            actions: Null
        };

        CREATE tb_notifications CONTENT noti_content;
        # RELATE $user_id -> tb_notifications -> $form_id SET date = time::now(), noti_title = "Your Id form has been rejected", noti_message = $noti_message, noti_type = "IdVerifyFormDenied";
    } ELSE IF $after.status_type IS "Approved" THEN {
          UPDATE $user_id SET address = $after.address, id_approved = true;
          RELATE $user_id -> tb_id_verified -> $form_id SET date = time::now();
          LET $noti_content = {
              date: time::now(),
              noti_title: "Your Id form has been approved",
              noti_message: $noti_message,
              noti_type: "IdVerifyFormApproved",
              noti_for_user: $user_id,
              actions: Null
          };

          CREATE tb_notifications CONTENT noti_content;

          # RELATE $user_id -> tb_notifications -> $form_id SET date = time::now(), noti_title = "Your Id form has been approved", noti_message = $noti_message, noti_type = "IdVerifyFormApproved";
    } END;
};
