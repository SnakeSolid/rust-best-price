CREATE TABLE category (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE UNIQUE INDEX nx_category_name ON category ( name ) ;

CREATE TABLE shop (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE UNIQUE INDEX nx_shop_name ON shop ( name ) ;

CREATE TABLE product (
    id INTEGER PRIMARY KEY,
    shop_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    url TEXT NOT NULL,
    name TEXT NOT NULL
);

CREATE UNIQUE INDEX nx_product_url ON product ( url ) ;

CREATE TABLE product_price (
    id INTEGER PRIMARY KEY,
    product_id INTEGER NOT NULL,
    iteration INTEGER NOT NULL,
    timestamp INTEGER NOT NULL,
    price REAL NOT NULL
);

CREATE INDEX nx_product_price_iteration ON product_price ( iteration ) ;

CREATE TABLE iteration (
    id INTEGER PRIMARY KEY,
    iteration INTEGER NOT NULL
);

INSERT INTO iteration ( iteration ) VALUES ( 0 ) ;
