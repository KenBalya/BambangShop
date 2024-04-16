use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::product::Product;

// Singleton of Database
lazy_static! {
    static ref PRODUCTS: DashMap<usize, Product> = DashMap::new();
}

pub struct ProductRepository;

impl ProductRepository {
    pub fn add(mut product: Product) -> Product {
        product.id = PRODUCTS.len();
        let product_value = product.clone();
        PRODUCTS.insert(product_value.id, product_value);
        return product;
    }

    pub fn list_all() -> Vec<Product> {
        return PRODUCTS.iter().map(|f| f.value().clone()).collect();
    }

    pub fn get_by_id(id: usize) -> Option<Product> {
        let result = PRODUCTS.get(&id);
        if !result.is_none() {
            return Some(result.unwrap().clone());
        }
        return None;
    }

    pub fn delete(id: usize) -> Option<Product> {
        let result = PRODUCTS.remove(&id);
        if !result.is_none() {
            return Some(result.unwrap().1);
        }
        return None;
    }
    pub fn update(id: usize, new_product_data: Product) -> Option<Product> {
        // Check if the product with the given id exists
        if let Some(mut product) = PRODUCTS.get_mut(&id) {
            // Update the product fields with new data
            product.title = new_product_data.title;
            product.description = new_product_data.description;
            product.price = new_product_data.price;
            product.product_type = new_product_data.product_type.to_uppercase();

            Some(product.clone())
        } else {
            None
        }
    }
}
