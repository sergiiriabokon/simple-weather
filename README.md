Simple Weather CLI
------------------

Allows to discover weather forecasts and current for different cities. Supports two providers:
**OpenWeather** and **WeatherAPI**.

Runnig the tool requires registration at those provider web-sites. After registering API key should be entered via *configure* command. 

The main commands are:

```
weather configure <provider>
weather get <provider> <city> [days]
```

*configure* - allows to enter provider API key.
*get* retrieves weather forecast for city via provider.

Examples:

```
weather configure openweather
weather configure weatherapi
weather get weatherapi Toledo 2
weather get openweather Toledo
```

Configurations are written to default *conf_dir*. On MacOS located at *~/Library/Application Support*

Building
--------
Building process downloads all the dependent libraries and push the results to *target/debug* folder.

```
cargo build
```

Running
-------
Running is possible either with build and run:
```
cargo run configure openweather
cargo run get weatherapi Toledo 2
```

or directly from *target/debug* folder

```
./target/debug/simple-weather configure weatherapi
./target/debug/simple-weather get weatherapi Toledo 2
```

Testing
-------
Tests should be performed after writing API keys through *configure* command.
```
cargo test
```
