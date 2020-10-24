use std::fs;
use failure::Fallible;
use headless_chrome::{Browser, protocol::{target::methods::CreateTarget, page::ScreenshotFormat}};

use mysql::*;
use mysql::prelude::*;



fn main() -> Fallible<()> {

    // Create a headless browser, navigate to wikipedia.org, wait for the page
    // to render completely, take a screenshot of the entire page
    // in JPEG-format using 75% quality.

    let width = 1024;
    let height = 800;

    let browser = Browser::default()?;
//    let tab = browser.wait_for_initial_tab()?;
    let tab = browser.new_tab_with_options(CreateTarget {
        url: "https://en.wikipedia.org/wiki/WebKit",
        width: Some(width.into()),
        height: Some(height.into()),
        browser_context_id: None,
        enable_begin_frame_control: None,
    })?;  

    /*
    let jpeg_data = tab
        .navigate_to("https://www.wikipedia.org")?
        .wait_until_navigated()?
        .capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;
    fs::write("screenshot.jpg", &jpeg_data)?;
    */ 

    // Browse to the WebKit-Page and take a screenshot of the infobox.
    let png_data = tab
        .navigate_to("https://en.wikipedia.org/wiki/WebKit")?
     //   .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
        .wait_until_navigated()?
        .capture_screenshot(ScreenshotFormat::PNG, None, true)?;

    fs::write("screenshot.png", &png_data)?;

    println!("Screenshots successfully created.");
    Ok(())
}


