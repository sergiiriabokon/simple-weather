use std::env;
use system_config::Config;
extern crate dirs;
extern crate reqwest;
use std::error::Error;
use serde_json::Value;
use std::io;
use std::fmt;

/**
 * Filename for the configuration file.
 */
const CONF_FILE: &str = "weather-config";

/**
 * Name for OpenWeather provider
 */
const OPENWEATHER: &str = "openweather";

/**
 * Name for WeatherAPI provider
 */
const WEATHERAPI: &str = "weatherapi";

/**
 * Occures while running with empty configuration 
 */
#[derive(Debug)]
struct EmptyConf;

impl fmt::Display for EmptyConf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "empty configuration file")
    }
}

impl Error for EmptyConf {}

/**
 * Occurs when configurations files directory is not found
 */
#[derive(Debug)]
struct NoConfDir;

impl fmt::Display for NoConfDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No configuration directory available")
    }
}

impl Error for NoConfDir {}

/**
 * Parses a provider name for command line arguments
 * 
 * @return String name of the provider (weatherapi or openweather)
 */
fn get_provider_from_args() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("no provider name specified!");
    }

    args[2].to_string()
}

/**
 * Parses a city from CLI arguments
 * 
 * @return String city name (like Toledo)
 */
fn get_city_from_args() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        panic!("no provider name specified!");
    }

    args[3].to_string()
}

/**
 * Parses days from CLI arguments
 * 
 * @return i32 number of days
 */
fn get_days_from_args() -> i32  {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        return 0;
    }

    args[4]
      .parse()
      .unwrap_or_else(
        |e| {
            println!("warning: could not parse days from arguments, using default zero {:?}", e); 
            0
        })
}


/**
 * Implements configure command. Example:
 * weather configure weatherapi
 */
fn configure() -> Result<(), Box<dyn Error>> {
    let provider = get_provider_from_args();

    println!("Please, provide API key for the provider {}", provider);

    let mut apikey = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut apikey);

    let mut config = Config::new(CONF_FILE).map_err(|e| format!("Failed to open conf file: {:?}", e))?;

    config.insert(provider, apikey.trim().to_string());
    config.write();


    println!("Saved to file {} in {}", 
        CONF_FILE, 
        dirs::config_dir().ok_or(NoConfDir)?.as_path().display());

    Ok(())
}

/**
 * Implementation of OpenWeather provider.
 */
fn get_open_weather(city: &String, _: i32) -> Result<(), Box<dyn Error>> {
    let provider = OPENWEATHER;

    let config = Config::new(CONF_FILE)?;

    let apikey = config.get(provider).ok_or(EmptyConf)?;

    let openweathquery = "http://api.openweathermap.org/data/2.5/weather?q=".to_owned() + 
                            &city + 
                            "&APPID=" +
                            &apikey;

    // println!("{}", &openweathquery);

    let owresp = reqwest::blocking::get(openweathquery)?.text()?;
    //println!("{:#?}", owresp);

    let v: Value = serde_json::from_str(&owresp)?;
    println!("Current weather in {} is {}", city, v["weather"][0]["description"]);

    Ok(())
}

/**
 * Implementation of WeatherAPI provider.
 */
fn get_weather_api(city: &String, days: i32) -> Result<(), Box<dyn Error>> {
    let provider = WEATHERAPI;

    let config = Config::new(CONF_FILE).map_err(|e| format!("Failed to open conf file: {:?}", e))?;

    let apikey = config.get(provider).ok_or(EmptyConf)?;

    let wquery = "http://api.weatherapi.com/v1/forecast.json?key=".to_owned() + 
                 &apikey + 
                 "&q=" + 
                 &city + 
                 "&days=" +
                 &days.to_string() +
                 "&aqi=no&alerts=no";

    // println!("{:#?}", wquery);
    let wresp = reqwest::blocking::get(wquery)?.text()?;
    // println!("{:#?}", wresp);

    let vw: Value = serde_json::from_str(&wresp)?;
    println!("Current weather in {} is {}", city, vw["current"]["condition"]["text"]);

    if days >= 2 {
        for i in 0 .. (days as usize) {
            println!("forecast for day {} is {}", 
                vw["forecast"]["forecastday"][i]["date"], 
                vw["forecast"]["forecastday"][i]["day"]["condition"]["text"]);
        }
    }

    Ok(())
}

/**
 * Implements get command. Example:
 * weather get weatherapi Toledo 1
 */
fn weather() -> Result<(), Box<dyn Error>> {
    let provider = get_provider_from_args();
    let city = get_city_from_args();
    let days = get_days_from_args();

    match provider.as_str().trim() {
        OPENWEATHER => get_open_weather(&city, days),
        WEATHERAPI => get_weather_api(&city, days),
        _ => Ok(println!("unkown provider")),
    }
}

/**
 * Entry point to the application
 */
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("no command specified");
    }

    let cmd = &args[1];

    match cmd.as_str().trim() {
        "configure" => configure(),
        "get"       => weather(),
        _           => Ok(println!("unnkown command")),
    }
}

/**
 * Unit and integration tests
 */
#[cfg(test)]
mod tests {
   use super::*;

    #[test]
    fn test_configurations_openweather() {
        let config = Config::new(CONF_FILE).unwrap_or_else(|_| {panic!("conf file not opened!")});

        let owapikey = config.get("openweather").unwrap_or_else(|| {
            println!("no open weather configured");
            "".to_string()
        });

        assert!(owapikey.len() > 0, "no configuration for open weather");
    }

    #[test]
    fn test_configurations_weatherapi() {
        let config = Config::new(CONF_FILE).unwrap_or_else(|_| {panic!("conf file not opened!")});

        let wapikey = config.get("weatherapi").unwrap_or_else(|| {
            println!("no weather api configured");
            "".to_string()
        });

        assert!(wapikey.len() > 0, "no configuration for weather api");
    }

    #[test]
    fn test_weatherapi() {
        let r = get_weather_api(&"Toledo".to_string(), 2);
        assert!(r.is_ok());
    }
    

    #[test]
    fn test_openweather() {
        let r = get_open_weather(&"Toledo".to_string(), 2);
        assert!(r.is_ok());
    }
}