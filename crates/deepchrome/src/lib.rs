use std::error::Error;

use headless_chrome::Browser;
use headless_chrome::protocol::cdp::Page;

pub fn launch_yuanbao() -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;
    let version_info = browser.get_version()?;
    println!("User-Agent is `{}`", version_info.user_agent);

    let tab = browser.new_tab()?;
    tab.set_default_timeout(std::time::Duration::from_secs(60*3));
    tab.navigate_to("https://yuanbao.tencent.com")?;

    tab.wait_until_navigated()?;

    let qrcode = tab.wait_for_element("#tpl_iframe > div.web_qrcode_panel > div.web_qrcode_initial_context.js_status.js_wx_default_tip > div > img")?;
    let screenshot = qrcode.capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
    std::fs::write("deepseek.jpeg", screenshot)?;
    Ok(())
}

pub fn launch_deepseek() -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;
    let version_info = browser.get_version()?;
    println!("User-Agent is `{}`", version_info.user_agent);

    let tab = browser.new_tab()?;
    tab.set_default_timeout(std::time::Duration::from_secs(60*3));
    tab.navigate_to("https://chat.deepseek.com")?;

    tab.wait_until_navigated()?;

    let qrcode = tab.wait_for_element("#tpl_iframe > div.web_qrcode_panel > div.web_qrcode_initial_context.js_status.js_wx_default_tip > div > img")?;
    let screenshot = qrcode.capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
    std::fs::write("deepseek.jpeg", screenshot)?;
    /*
    let box_model = qrcode.get_box_model()?;

    let screenshot = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Png,
        None,
        Some(Page::Viewport {
            x: box_model.content.into_iter().nth(0).unwrap() as f64,
            y: box_model.content.into_iter().nth(1).unwrap() as f64,
            width: box_model.width as f64,
            height: box_model.height as f64,
            scale: 1.0,
        }),
        true,
    )?;
    */
    /*
    tab.wait_for_element("#root > div > div > div._99ad066 > div > div > div.ds-sign-up-form__main > div.ds-sign-up-form__main-hero > div:nth-child(3) > div.ds-form-item__content > div")?;

    let jpeg_data = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Jpeg,
        None,
        None,
        true)?;
    std::fs::write("deepseek.jpeg", jpeg_data)?;
    */
    Ok(())
}
