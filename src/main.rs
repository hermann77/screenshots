use std::fs;
use std::path::Path;
use std::time::Instant;
use failure::Fallible;
use headless_chrome::{Browser, protocol::{target::methods::CreateTarget, page::ScreenshotFormat}};

use mysql::Pool;


#[derive(Debug, PartialEq, Eq)]
struct Bookmark {
    bid: i32,
    edutags_url: String
}

fn main() -> Fallible<()> {

    let start_time = Instant::now();

    let bookmarks = select_db();

    // Create a headless browser, navigate to wikipedia.org, wait for the page
    // to render completely, take a screenshot of the entire page
    // in JPEG-format using 75% quality.

    let width = 1024;
    let height = 800;

    let browser = Browser::default()?;


    for bookmark in bookmarks.iter() {
        println!("BID: {}, URL: {}", bookmark.bid, bookmark.edutags_url);
    
        //    let tab = browser.wait_for_initial_tab()?;
        let tab = browser.new_tab_with_options(CreateTarget {
            url: &bookmark.edutags_url,
            width: Some(width.into()),
            height: Some(height.into()),
            browser_context_id: None,
            enable_begin_frame_control: None,
        })?;  

        println!("Tab created");

        /*
        let jpeg_data = tab
            .navigate_to("https://www.wikipedia.org")?
            .wait_until_navigated()?
            .capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;
        fs::write("screenshot.jpg", &jpeg_data)?;
        */ 

        // Browse to the WebKit-Page and take a screenshot of the infobox.
        let png_data = tab
        //    .navigate_to(&bookmark.edutags_url)?
        //   .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
            .wait_until_navigated()?
            .capture_screenshot(ScreenshotFormat::PNG, None, true)?;

        let mut filename: String = bookmark.bid.to_string().to_owned();
        let file_ext: &str = ".png";
        filename.push_str(file_ext);
        let path = Path::new(&filename);

        fs::write(path, &png_data)?;
    }


    println!("Screenshots successfully created.");

    let elapsed = start_time.elapsed();
    println!("Elapsed time {:?}", elapsed);

    Ok(())
}



fn select_db() -> Vec<Bookmark> {

    let db_url = "mysql://root:@localhost:3307/drupal_edutags_d8";
    let pool = Pool::new(db_url).unwrap();


    let bookmarks: Vec<Bookmark> = pool.prep_exec(
        "SELECT bid AS bid, url AS edutagsURL FROM bookmark WHERE url IS NOT NULL AND url <> '' AND bid >= 12 AND bid <= 20", ())
        .map(|result| {
            result.map(|x| x.unwrap())
            .map(|row| {
                let (bid, edutags_url) = mysql::from_row(row);

                Bookmark {
                    bid,
                    edutags_url
                }
            }).collect()
        }).unwrap();

    return bookmarks;
}
