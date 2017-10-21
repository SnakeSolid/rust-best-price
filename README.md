# Best Price

Crawler and WEB server to find most cheaper things in online shops.

## Quick Start
[quick-start]: #quick-start

To build `best-price` from source code use following command:

```sh
cargo build --release
```

To start `best-price` with configuration file `example.toml` listening on `localhost:8080` use following command:

```sh
./target/release/best-price -r -c example.toml
```

Local database will be created in file `local.sqlite`. WEB server will be available on
[localhost|http://localhost:8080/].

## Commandline options
[commandline-options]: #commandline-options

* `-h` (`--help`) - show short description for all commend line options and exit;
* `-b HOST` (`--bind HOST`), optional - address to bind WEB server on. Default value: `localhost`;
* `-p PORT` (`--port PORT`), optional - port to listen for WEB server. Default value: `8080`;
* `-e PERIOD` (`--period PERIOD`), optional - period to update price for all products. Parameter is integer number of
	hours between updates. Default value: `12`;
* `-c FILE` (`--config FILE`), optional - path to configuration file. Detailed information about configuration file
	content see in [configuration] section. Default value: `config.toml`;
* `-d FILE` (`--database FILE`), optional - path to local SQLite database to store parsed prices in. Default value:
	`local.sqlite`;
* `-r` (`--create`), optional - if this option present local database will be created after start. If database already
	exists try to initialize schema in this database;
* `-f` (`--force`), optional - if this option present local database will be removed and created again;
* `-s` (`--disable-crawler`), optional - if this option present background crawler will not be started.

## Configuration
[configuration]: #configuration

Configuration file must be written in `toml` format. Configuration file has two main sections: `shops` and `products`.

### Shops section
[shops-section]: #shops-section

This section describes all shops and shops related data. Every shop has three required parameters: `name`,
`name_selector` and `price_selector`.

* parameter `name` represents shop name. It will be displayed in frontend. Also products associates with shop through
	the shop name;
* parameter `name_selector` contains valid CSS selector to product name element on a page. If selector match several
	elements first non empty will be chosen. If name not found on a page - page consider as invalid;
* parameter `price_selector` contains valid CSS selector to price element on a page. Selector can match several
	elements. Selected price element depends on `price_index` parameter. If price not found on a page - page consider
	as invalid.

Optional shop parameters:

* parameter `price_factor` shows multiplier for price. During parsing price only digits retained from string.
	Eventually product price represented by integer number. Multiplier `price_factor` can convert this number to real
	price or different currency;
* parameter `price_index` define which price should be selected. If page contains several price block this parameter
	point to particular price block to choose;
* parameter `cookies` contains cookies for site. Cookies can be used for authorization on the site or adding some
	specific options like city.

By default if required parameter is not specified default value will be used. Default values:

* `price_factor = 1.0`;
* `price_index = 1`;
* `cookies` are empty.

### Products section
[products-section]: #products-section

This section describes product related data. Every product has three required parameters: `shop_name`, `category` and
`url`. Products have no optional parameter.

* parameter `shop_name` represents shop name. Product will be associated with this shop. All page pasring options will
	be taken from shop description;
* parameter `category` product category name. Products with same category will be in same chart on prices page. Also
	most cheaper product will be shown in the best products table;
* parameter `url` contains valid URL to product page. Internally URL is unique identifier of the product.

## Configuration example
[configuration-example]: #configuration-example

```toml
[[shops]]
name = "Amazon"
name_selector = "#productTitle"
price_selector = "span.header-price"
price_factor = 0.01

[shops.cookies]
# cookie value can differs
ubid-main = "132-9149479-7262533"

[[products]]
shop_name = "Amazon"
category = "book"
url = "https://www.amazon.com/Programming-Rust-Fast-Systems-Development/dp/1491927283"
```

## License
[license]: #license

Source code is primarily distributed under the terms of the MIT license. See LICENSE for details.