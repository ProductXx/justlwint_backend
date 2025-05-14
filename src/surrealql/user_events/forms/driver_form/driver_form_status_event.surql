DEFINE EVENT IF NOT EXISTS driver_form_status ON TABLE tb_driver_forms WHEN $event = "UPDATE" THEN {
    LET $user_id = $after.user_id;
    LET $form_id = $after.id;
    LET $noti_message = $after.message;
    IF $after.status_type IS "Rejected" THEN {
        LET $noti_content = {
            date: time::now(),
            noti_title: "Your Driver form has been rejected",
            noti_message: $noti_message,
            noti_type: "DriverFormDenied",
            noti_for_user: $user_id,
            actions: Null
        };

        CREATE tb_notifications CONTENT noti_content;
        # RELATE $user_id -> tb_notifications -> $form_id SET date = time::now(), noti_title = "Your Driver form has been rejected", noti_message = $noti_message, noti_type = "DriverFormDenied";
    } ELSE IF $after.status_type IS "Approved" THEN {
          UPDATE $user_id SET is_driver = true;
          LET $driver_id = type::thing("tb_driver", $after.id.id);
          CREATE $driver_id SET fullname = $after.username, phone_number = $after.phone_number, experience_details = $after.experience_details, is_available = true, is_ongoing = false;

          LET $noti_content = {
            date: time::now(),
            noti_title: "Your Driver form has been approved",
            noti_message: $noti_message,
            noti_type: "DriverFormApproved",
            noti_for_user: $user_id,
            actions: Null
          };

          RELATE $user_id -> tb_registered_drivers -> $driver_id SET date = time::now();
          CREATE tb_notifications CONTENT noti_content;
          # RELATE $user_id -> tb_notifications -> $form_id SET date = time::now(), noti_title = "Your Driver form has been approved", noti_message = $noti_message, noti_type = "DriverFormApproved";
    } END;
};
