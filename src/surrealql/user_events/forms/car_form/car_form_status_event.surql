DEFINE EVENT IF NOT EXISTS car_form_status ON TABLE tb_car_forms WHEN $event = "UPDATE" THEN {
    LET $user_id = $after.user_id;
    LET $form_id = $after.id;
    LET $noti_message = $after.message;
    IF $after.status_type IS "Rejected" THEN {
        LET $noti_content = {
            date: time::now(),
            noti_title: "Your Car form has been rejected",
            noti_message: $noti_message,
            noti_type: "CarFormDenied",
            noti_for_user: $user_id,
            actions: Null
        };

        CREATE tb_notifications CONTENT $noti_content;
        # RELATE $user_id -> tb_notifications -> $form_id SET date = time::now(), noti_title = "Your Car form has been rejected", noti_message = $noti_message, noti_type = "CarFormDenied";
    } ELSE IF $after.status_type IS "Approved" THEN {
          IF !$user_id.is_owner THEN {
             UPDATE $user_id SET is_owner = true;
            } END;

          LET $car_id = type::thing("tb_car", $after.id.id);
          LET $noti_content = {
            date: time::now(),
            noti_title: "Your Car form has been approved",
            noti_type: "CarFormApproved",
            noti_message: $noti_message,
            noti_for_user: $user_id,
            actions: Null
          };

          CREATE $car_id SET owner_id = $user_id, owner_name = $user_id.username, model = $after.model, license_number = $after.license_number, details = $after.details, car_photos = $after.car_photos, is_available = false, is_ongoing = false;
          RELATE $user_id -> tb_own_car -> $car_id SET date = time::now();

          CREATE tb_notifications CONTENT $noti_content;
          # RELATE $user_id -> tb_notifications -> $form_id SET date = time::now(), noti_title = "Your Car form has been approved", noti_message = $noti_message, noti_type = "CarFormApproved";

    } END;
};

