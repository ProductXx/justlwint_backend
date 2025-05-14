DEFINE EVENT IF NOT EXISTS post_upload ON TABLE tb_booking WHEN $event = "CREATE" THEN {
    LET $noti_type = IF $after.x_is_booing_the IS 'Car' {
        'PackageBookingTheCar'
    } ELSE IF $after.x_is_booing_the IS 'Package' {
        'CarBookingThePackage'
    };

    LET $noti_content = {
        date: time::now(),
        noti_title: string::concat($after.booker_id.username, " is wanting to book your ", $after.owner_post_id.title, " for its ", $after.booker_post_id.title),
        noti_type: $noti_type,
        noti_message: Null,
        noti_for_user: $after.owner_id,
        actions: [
          {action_label: "Details", url: string::concat("/fetch/fetch_post/", $after.booker_post_id.id.id)},
          {action_label: "Accept", url: string::concat("/booking/accept/", $after.id.id)},
          {action_label: "Deny", url: string::concat("/booking/deny/", $after.id.id)},
        ]
    };

    CREATE tb_notifications CONTENT $noti_content;
};
