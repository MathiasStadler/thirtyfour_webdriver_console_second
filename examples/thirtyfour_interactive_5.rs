// update here
// RUN PRG /W full log output
// RUST_LOG=debug cargo run --example thirtyfour_get_margin_data_twenty_four 2>&1 | tee output1.txt

// start cleanup

// RUSTFLAGS=-Wunused-crate-dependencies cargo

// env_logger
// RUST_LOG=info ./main

// thirtyfour 2024
// https://www.zenrows.com/blog/rust-selenium#install-selenium
#[allow(unused_imports)]
use log::{debug, error, info, log_enabled, Level};

// use async_recursion::async_recursion;

//
use std::env::set_var;

use std::error::Error;

use std::io::Write;
use std::process;

use std::thread;
use std::time::Duration;

use std::io::stdin;
use std::io::stdout;


use std::fmt;

#[allow(unused_imports)]
use thirtyfour::ChromiumLikeCapabilities;
#[allow(unused_imports)]
use thirtyfour::{prelude::WebDriverError, By, DesiredCapabilities, Key, WebDriver, WebElement};

// pub type WebDriverResult<T> = Result<T, WebDriverError>;

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

fn main() -> Result<(), Box<dyn Error>> {
    set_var("RUST_LOG", "debug");
    // color_eyre::install()?;

    let mut _call_counter: i32;

    env_logger::builder()
        .format(|buf, record| {
            let warn_style = buf.default_level_style(log::Level::Warn);
            let _timestamp = buf.timestamp();
            writeln!(
                buf,
                // FROM HERE
                // https://docs.rs/env_logger/latest/src/custom_format/custom_format.rs.html#35
                "{}:{}  {warn_style}{}{warn_style:#} - {}",
                // record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                // chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args(),
            )
        })
        .init();

    error!("RUST_LOG maybe NOT enable");
    error!("Used: => RUST_LOG=info < prg >");

    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let _ = rt.block_on(run());

    info!("env_logger: ended");
    process::exit(0);
}

async fn run() -> Result<(), Box<dyn Error>> {
    let _place: &str = "Place";

    // path_to().await?;
    action_interactive().await?;

    Ok(())
}

#[allow(dead_code)]
async fn close_browser(_driver: WebDriver) -> Result<(), Box<dyn Error>> {
    // Always explicitly close the browser.
    _driver.quit().await?;

    Ok(())
}

// https://github.com/stevepryde/thirtyfour/issues/4?ref=https://githubhelp.com
//
#[allow(dead_code)]
async fn wait_seconds_of_browser(waiting_period: u64) -> Result<(), Box<dyn Error>> {
    // debug!("wait for page completed load => wait for status from chrome driver");
    // debug!("driver=> {:?}", _driver.status().await?);
    debug!("Thread sleep for {} seconds", waiting_period);
    thread::sleep(Duration::from_secs(waiting_period));
    Ok(())
}

async fn action_interactive() -> Result<(), Box<dyn Error>> {
    loop {
        print!("> ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string();

        debug!("input => {}", input);

        // shell exit
        if input == "exit" {
            break;
        }
        //end if input == "exit"

        if input.starts_with(':') {
            debug!("input command modus => {}", input);
            if !input.is_empty() {
                if input.len() > 1 {
                    input.remove(0); // remove first sign, the double point
                    debug!("plain command => {}", input);

                    let _execute_command_result = execute_command(&input).await;

                    let _ = match _execute_command_result {
                        //everything is fine
                        Ok(()) => (),
                        Err(_e) => {
                            return Err(Box::new(MyError(
                                "Error _execute_command => {_e}".to_string(),
                            ))
                            .into())
                        }
                    };
                } else {
                    debug!("input no command  => {}", input);
                }
            } else {
                error!("input size is zero");
            }

            debug!("leave command modus => {}", input);
        }
    }
    // old debug!("Thread sleep for {} seconds", waiting_period);
    // old thread::sleep(Duration::from_secs(waiting_period));
    Ok(())
}

async fn execute_command(cmd: &String) -> Result<(), Box<dyn Error>> {
    info!("start => execute_command -> {}", cmd);
    // let _driver:: <Result<WebDriver::WebDriverError>> = std::option::Option ;

    // debug!("execute_command  _cmd => {}", cmd);

    // if cmd == "init" {
    debug!("execute_command  _cmd => {}", cmd);
    let _driver = init_driver().await?;

    // let _result_init_driver = init_driver();
    // } else
    if cmd == "close" {
        debug!("execute_command  _cmd => {}", cmd);
        let result_close_browser = close_browser(_driver.clone()).await;
        let _ = match result_close_browser {
            Ok(_web_element) => {
                info!(r#"ACTION_BROWSER_CLOSE => Ok"#);
            }
            Err(_e) => {
                error!(r#"ACTION_BROWSER_CLOSE => Err {_e}"#);
            }
        };

        // let _result_init_driver = init_driver();
    } else if cmd == "open" {
        debug!("execute_command  cmd => {}", cmd);
    } else {
        info!("Opps!!! Command NOT FOUND {}", cmd);
    }
    debug!("finished => execute_command -> {}", cmd);

    Ok(())
}

// 25 year
// https://www.macrotrends.net/stocks/charts/TREX/trex/stock-price-history
// sec data
// https://www.sec.gov/cgi-bin/viewer?action=view&cik=1069878&accession_number=0001193125-23-266276&xbrl_type=v#

// FOUND HERE
// https://itehax.com/blog/web-scraping-using-rust

async fn init_driver() -> Result<WebDriver, WebDriverError> {
    info!("init_driver - start");

    let mut _caps = DesiredCapabilities::chrome();

    _caps.add_arg("--remote-debugging-pipe")?;
    _caps.add_arg("--no-sandbox")?;

    let driver_result = WebDriver::new("http://localhost:9515", _caps).await;

    // let result = WebDriver::new("http://localhost:4444/wd/hub", &caps).await;
    let driver = match driver_result {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    driver.maximize_window().await?;
    info!("init_driver - end");
    Ok(driver)
}

/*
rustfmt  ./examples/thirtyfour_interactive_5.rs

cargo build --example thirtyfour_interactive_5
cargo run --example thirtyfour_interactive_5
*/
