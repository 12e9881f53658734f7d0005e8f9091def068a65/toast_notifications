#![windows_subsystem = "windows"]
use windows::{
    UI::Notifications::{ToastNotificationManager, ToastNotification},
    Data::Xml::Dom::XmlDocument,
    core::HSTRING
};

fn main() {
    let xml = XmlDocument::new().unwrap();
    xml.LoadXml(&HSTRING::from(r#"
    <toast>
        <visual>
            <binding template="ToastImageAndText01">
                <image id="1" src=""/>
                <text id="1">Blocked 9 ADS so far.</text>
            </binding>
        </visual>
    </toast>
"#)).unwrap();
    let r = ToastNotification::CreateToastNotification(&xml).unwrap();
    
    let test: HSTRING = "Chrome".into(); //Title

    let manager =  ToastNotificationManager::CreateToastNotifierWithId(&test).unwrap();

    manager.Show(&r).unwrap();
}