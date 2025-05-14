DEFINE EVENT IF NOT EXISTS car_post_status ON TABLE tb_car_post_driver WHEN $event = "UPDATE" THEN {
    LET $owner_id = $after.owner_id;
    LET $driver_id = $after.driver_id;
    LET $car_id = $after.car_id;
    LET $form_id = $after.id;
    LET $noti_message = $after.message;
    IF $after.status_type IS "Deny" THEN {
        LET $owner_noti_content = {
            date: time::now(),
            noti_title: "The Driver is denied to drive your car.",
            noti_type: 'DriverDeniedToDrive',
            noti_message: Null,
            noti_for_user: $owner_id, 
            actions: Null
        };

        CREATE tb_notifications CONTENT $owner_noti_content;
        DELETE $form_id;
    } ELSE IF $after.status_type IS "Accept" THEN {
        UPDATE $driver_id SET is_available = false;
        UPDATE $car_id SET is_available = true;

        LET $post_content = {
            date: time::now(),
            post_type: 'Car',
            title: $after.title,
            description: $car_id.details,
            author: $owner_id.name,
            author_id: $owner_id,
            car_id: $car_id,
            driver_name: $driver_id.name,
            driver_id: $driver_id,
            start_location_detail: $after.start_location_detail,
            end_location_detail: $after.end_location_detail,
            start_location: $after.start_location,
            end_location: $after.end_location,
            expected_date_to_start: $after.expected_date_to_start,
            post_photos: $car_id.car_photos,
            main_package_types: $after.main_package_types, 
            cost_per_type: $after.cost_per_type,
            available: true,
        };

        LET $post_id = type::thing("tb_posts", $form_id.id);

        CREATE $post_id CONTENT $post_content;
        DELETE $form_id;

        RELATE $car_id -> tb_car_posts -> $post_id SET date = time::now();
        RELATE $driver_id -> tb_driving -> $post_id SET date = time::now();

        LET $owner_noti_content = {
            date: time::now(),
            noti_title: "The Driver is accepted to driver your car.",
            noti_type: 'DriverAcceptedToDrive',
            noti_message: Null,
            noti_for_user: $owner_id,
            actions: Null
        };
        
        CREATE tb_notifications CONTENT $owner_noti_content;

        LET $car_ongoing_content = {
          date: time::now(),
          driver_id: $driver_id,
          owner_id: $owner_id,
          car_post_id: $post_id,
          booked_packages_list: [],
        };
        
        CREATE type::thing("tb_car_ongoing", $car_id.id) CONTENT car_ongoing_content;
    } END;
};
