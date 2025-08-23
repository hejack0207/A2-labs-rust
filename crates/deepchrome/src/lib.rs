use std::error::Error;

use headless_chrome::Browser;
use headless_chrome::protocol::cdp::Page;

pub fn launch_deepseek() -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;

    let tab = browser.new_tab()?;
    tab.navigate_to("https://chat.deepseek.com")?;

    tab.wait_for_element("#root > div > div > div._99ad066 > div > div > div.ds-sign-up-form__main > div.ds-sign-up-form__main-hero > div:nth-child(3) > div.ds-form-item__content > div")?.click()?;
    Ok(())
}
