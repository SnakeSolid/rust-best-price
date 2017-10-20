#[derive(Debug, Clone)]
pub struct Product {
    id: i64,
    name: String,
}


impl Product {
    #[inline]
    pub fn new(id: i64, name: String) -> Product {
        Product { id, name }
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
pub struct ProductPrice {
    iteration: i64,
    timestamp: i64,
    price: f64,
}


impl ProductPrice {
    #[inline]
    pub fn new(iteration: i64, timestamp: i64, price: f64) -> ProductPrice {
        ProductPrice {
            iteration,
            timestamp,
            price,
        }
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


#[derive(Debug, Clone)]
pub struct IterationPrice {
    category_id: i64,
    category: String,
    product: String,
    url: String,
    shop: String,
    price: f64,
    timestamp: i64,
}


impl IterationPrice {
    #[inline]
    pub fn new(
        category_id: i64,
        category: String,
        product: String,
        url: String,
        shop: String,
        price: f64,
        timestamp: i64,
    ) -> IterationPrice {
        IterationPrice {
            category_id,
            category,
            product,
            url,
            shop,
            price,
            timestamp,
        }
    }

    #[inline]
    pub fn category_id(&self) -> i64 {
        self.category_id
    }

    #[inline]
    pub fn category(&self) -> String {
        self.category.clone()
    }

    #[inline]
    pub fn product(&self) -> String {
        self.product.clone()
    }

    #[inline]
    pub fn url(&self) -> String {
        self.url.clone()
    }

    #[inline]
    pub fn shop(&self) -> String {
        self.shop.clone()
    }

    #[inline]
    pub fn price(&self) -> f64 {
        self.price
    }

    #[inline]
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
}
