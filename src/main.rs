use std::fs;
use std::path::Path;
use std::time::Instant;
use failure::{Fallible};
use url::{Url};
use headless_chrome::{Browser, protocol::{target::methods::CreateTarget, page::ScreenshotFormat}};

use mysql::Pool;


#[derive(Debug, PartialEq, Eq)]
struct Bookmark {
    bid: i32,
    edutags_url: String
}

fn main() -> Fallible<()> {

    let start_time = Instant::now();

    let width = 1024;
    let height = 800;
    let browser = Browser::default()?;

    let mut bookmarks = select_db();

    for bookmark in bookmarks.iter_mut() {

        create_screenshot(bookmark, &browser).expect("TODO: panic message");
    }

    println!("Screenshots successfully created.");

    let elapsed = start_time.elapsed();
    println!("Elapsed time {:?}", elapsed);
    Ok(())
}


fn create_screenshot(bookmark: &mut Bookmark, browser: &Browser) -> Fallible<()> {

    let width = 1024;
    let height = 800;
    let url_size = bookmark.edutags_url.len();
    let file_ext = &bookmark.edutags_url[url_size-3..];
    let url = Url::parse(&bookmark.edutags_url)?;

    match file_ext == "pdf" {
        true => {
            match base_url(url) {
                Ok(v) => bookmark.edutags_url = String::from(v.as_str()),
                Err(e) => println!("base_url ERROR: {:?}", e)
            }
        }
        _ => {
            
        }
    }

    println!("BID: {}, URL: {}", bookmark.bid, bookmark.edutags_url);

    //    let tab = browser.wait_for_initial_tab()?;
    let tab = browser.new_tab_with_options(CreateTarget {
        url: &bookmark.edutags_url,
        width: Some(width.into()),
        height: Some(height.into()),
        browser_context_id: None,
        enable_begin_frame_control: None,
    })?; 

    // Browse to the WebKit-Page and take a screenshot of the infobox.
    match tab.navigate_to(&bookmark.edutags_url) {
        Ok(tab_nav) => println!("Navigate to URL {}", tab_nav.get_url()),
        Err(e) => {
            println!("URL is not reachable. Error {}", e);
        },
    };
    
    tab.wait_until_navigated()?;

    let png_data = tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?;

    let mut filename: String = bookmark.bid.to_string().to_owned();
    let file_ext: &str = ".png";
    filename.push_str(file_ext);
    let path = Path::new(&filename);

    fs::write(path, &png_data)?;

    Ok(())
}


fn select_db() -> Vec<Bookmark> {

    let db_url = "mysql://root:@localhost:3306/drupal_edutags_d9";
    let pool = Pool::new(db_url).unwrap();


    let bookmarks: Vec<Bookmark> = pool.prep_exec(
        "SELECT bid AS bid, url AS edutagsURL FROM bookmark WHERE url IS NOT NULL AND url <> '' AND bid >= 1 AND bid <= 20", ())
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


fn base_url(mut url: Url) -> Result<Url, &'static str> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return Err("Could not shorten URL");
        }
    }

    url.set_query(None);

    Ok(url)
}