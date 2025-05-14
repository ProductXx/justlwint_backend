DEFINE EVENT IF NOT EXISTS booking_status ON TABLE tb_booking WHEN $event = "UPDATE" THEN {
  LET $booker_id = $after.booker_id;
  LET $owner_id = $after.owner_id;
  LET $booktb_id = $after.id;

  IF $after.status IS 'Accept' {
    LET $booker_noti_content = {
      date: time::now(),
      noti_title: string::concat("Owner accepted your booking for ", $after.owner_post_id.title, "."),
      noti_type: 'BookingAccept',
      noti_message: Null,
      noti_for_user: $booker_id,
      actions: Null
    };

    CREATE tb_notifications CONTENT $booker_noti_content;

    IF $after.x_is_booking_the IS 'Car' {
      UPDATE type::thing("tb_car_ongoing", $after.owner_post_id.id.id) SET booked_packages_list += $after.booker_post_id;
    } ELSE IF $after.x_is_booking_the IS 'Package' {
      UPDATE type::thing("tb_car_ongoing", $after.booker_post_id.id.id) SET booked_packages_list += $after.owner_post_id;
    };

  } ELSE IF $after.status IS 'Deny' {
    LET $booker_noti_content = {
      date: time::now(),
      noti_title: string::concat("Owner denied your booking for ", $after.owner_post_id.title, "."),
      noti_type: 'BookingDeny',
      noti_message: Null,
      noti_for_user: $booker_id,
      actions: Null
    };

    CREATE tb_notifications CONTENT $booker_noti_content;
  };
};
