CREATE TABLE category (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE UNIQUE INDEX category_name ON category ( name ) ;

CREATE TABLE shop (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE UNIQUE INDEX shop_name ON shop ( name ) ;

CREATE TABLE product (
    id INTEGER PRIMARY KEY,
    shop_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    name TEXT
);

CREATE UNIQUE INDEX product_name ON product ( name ) ;

CREATE TABLE product_price (
    id INTEGER PRIMARY KEY,
    product_id INTEGER,
    timestamp INTEGER,
    price REAL
);
