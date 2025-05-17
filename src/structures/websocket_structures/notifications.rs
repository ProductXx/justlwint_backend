use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

#[derive(Serialize, Deserialize)]
pub struct UserNotification {
    pub date: Datetime,
    pub noti_title: String,
    pub noti_type: NotiType,
    pub noti_message: Option<String>,
    pub noti_for_user: RecordId,
    pub actions: Option<Vec<Action>>,
}

#[derive(Serialize, Deserialize)]
pub enum NotiType {
    PostUploaded,
    IdVerifyFormDenied,
    IdVerifyFormApproved,
    DriverFormDenied,
    DriverFormApproved,
    DriverDeniedToDrive,
    DriverAcceptedToDrive,
    AskDriverToDrive,
    CarFormDenied,
    CarFormApproved,
    PackageBookingTheCar,
    CarBookingThePackage,
}

#[derive(Serialize, Deserialize)]
pub struct Action {
    pub action_label: String,
    pub url: String,
}
