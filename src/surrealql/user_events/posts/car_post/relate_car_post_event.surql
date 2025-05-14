DEFINE EVENT IF NOT EXISTS relate_car_posts ON TABLE tb_car_post_driver WHEN $event = "CREATE" THEN {
    LET $driver_id = $after.driver_id;
    LET $owner_id = $after.owner_id;
    # LET $form_id = $after.id;
    LET $noti_content = {
        date: time::now(),
        noti_title: string::concat($owner_id.name, " want you to drive his car"),
        noti_message: Null,
        noti_type: "AskDriverToDrive",
        noti_for_user: $user_id,
        actions: [
          {action_label: "Accept", url: string::concat("/driver_ask/accept/", $after.id.id)},
          {action_label: "Deny", url: string::concat("/driver_ask/deny/", $after.id.id)},
        ]
    };

    CREATE tb_notifications CONTENT $noti_content;
    # RELATE $form_id -> tb_ask_driver -> $driver_id SET date = time::now();
};
