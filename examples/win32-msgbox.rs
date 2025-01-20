

use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml(h!("<html>hello world</html>"))?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    assert!(root.InnerText()? == "hello world");

    unsafe {
        let event = CreateEventW(None, true, false, None)?;
        SetEvent(event)?;
        WaitForSingleObject(event, 0);
        CloseHandle(event)?;

        let r1 = MessageBoxA(None, s!("Ansi"), s!("Caption 1"), MB_OK);
        println!("MessageBoxA returned {:?}", r1);
        let r2 = MessageBoxW(None, w!("Wide"), w!("Caption 2"), MB_OK);
        println!("MessageBoxW returned {:?}", r2);
    }

    Ok(())
}
