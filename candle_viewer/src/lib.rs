use webbrowser::{Browser, BrowserOptions};

pub fn view(url: &str) -> Result<(), String> {
    let mut o = BrowserOptions::default();
    let o = o.with_target_hint("_self");

    webbrowser::open_browser_with_options(Browser::default(), url, o).map_err(|e| e.to_string())?;

    Ok(())
}
