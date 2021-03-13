pub trait Notification {
    fn notify(&self) -> String;
}

pub struct Twitter;

impl Notification for Twitter {
    fn notify(&self) -> String {
        // here do something
        "notification with twitter".to_string()
    }
}

pub struct Facebook;

impl Notification for Facebook {
    fn notify(&self) -> String {
        // here do something
        "notification with facebook".to_string()
    }
}
