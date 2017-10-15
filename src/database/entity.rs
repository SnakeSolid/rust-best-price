#[derive(Debug, Clone)]
pub struct Category {
    id: i64,
    name: String,
}


impl Category {
    #[inline]
    pub fn new(id: i64, name: String) -> Category {
        Category { id, name }
    }

    #[inline]
    pub fn id(&self) -> i64 {
        self.id
    }

    #[inline]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}


#[derive(Debug, Clone)]
pub struct Shop {
    id: i64,
    name: String,
}


impl Shop {
    #[inline]
    pub fn new(id: i64, name: String) -> Shop {
        Shop { id, name }
    }

    #[inline]
    pub fn id(&self) -> i64 {
        self.id
    }

    #[inline]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}


#[derive(Debug, Clone)]
pub struct Product {
    id: i64,
    shop_id: i64,
    category_id: i64,
    url: String,
    name: String,
}


impl Product {
    #[inline]
    pub fn new(id: i64, shop_id: i64, category_id: i64, url: String, name: String) -> Product {
        Product {
            id,
            shop_id,
            category_id,
            url,
            name,
        }
    }

    #[inline]
    pub fn id(&self) -> i64 {
        self.id
    }

    #[inline]
    pub fn shop_id(&self) -> i64 {
        self.shop_id
    }

    #[inline]
    pub fn category_id(&self) -> i64 {
        self.category_id
    }

    #[inline]
    pub fn url(&self) -> String {
        self.url.clone()
    }

    #[inline]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}


#[derive(Debug, Clone)]
pub struct ProductPrice {
    id: i64,
    product_id: i64,
    iteration: i64,
    timestamp: i64,
    price: f64,
}


impl ProductPrice {
    #[inline]
    pub fn new(
        id: i64,
        product_id: i64,
        iteration: i64,
        timestamp: i64,
        price: f64,
    ) -> ProductPrice {
        ProductPrice {
            id,
            product_id,
            iteration,
            timestamp,
            price,
        }
    }

    #[inline]
    pub fn id(&self) -> i64 {
        self.id
    }

    #[inline]
    pub fn product_id(&self) -> i64 {
        self.product_id
    }

    #[inline]
    pub fn iteration(&self) -> i64 {
        self.iteration
    }

    #[inline]
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    #[inline]
    pub fn price(&self) -> f64 {
        self.price
    }
}
