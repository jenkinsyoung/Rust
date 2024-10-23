use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Product {
    #[allow(dead_code)]
    name: String,
    price: f64,
    quantity: u32,
}

struct Store {
    products: HashMap<String, Product>,
}

impl Store {
    fn new() -> Store {
        Store {
            products: HashMap::new(),
        }
    }

    fn add_product(&mut self, name: String, price: f64, quantity: u32) -> Result<(), &'static str> {
        // Реализуйте добавление товара с проверкой корректности данных
        if price < 0.0 {
            return Err("Price cannot be negative");
        }
        if quantity == 0 {
            return Err("Quantity must be greater than zero");
        }

        if self.products.contains_key(&name) {
            return Err("Product already exists");
        }

        let product = Product { name: name.clone(), price, quantity };
        self.products.insert(name, product);
        Ok(())
    }

    fn update_product(&mut self, name: &str, new_price: Option<f64>, new_quantity: Option<u32>) -> Result<(), &'static str> {
        // Реализуйте обновление данных о товаре с проверкой наличия товара
        if let Some(product) = self.products.get_mut(name) {
            if let Some(price) = new_price {
                if price < 0.0 {
                    return Err("Price cannot be negative");
                }
                product.price = price;
            }
            if let Some(quantity) = new_quantity {
                if quantity == 0 {
                    return Err("Quantity must be greater than zero");
                }
                product.quantity = quantity;
            }
            Ok(())
        } else {
            Err("Product is not found")
        }
    }

    fn find_product(&self, name: &str) -> Option<&Product> {
        // Реализуйте поиск товара по названию
        self.products.get(name)
    }

    fn remove_product(&mut self, name: &str) -> Result<(), &'static str> {
        // Реализуйте удаление товара с проверкой наличия товара
        if let Some(_product) = self.products.get_mut(name) {
            self.products.remove(name);
            Ok(())
        } else {
            Err("Product is not found")
        }
    }
}

fn main() {
    let mut store = Store::new();
    
    // Попытка добавить товар
    match store.add_product("Laptop".to_string(), 1000.0, 10) {
        Ok(_) => println!("Product added successfully"),
        Err(e) => println!("Error adding product: {}", e),
    }
    // Поиск товара
    if let Some(product) = store.find_product("Laptop") {
        println!("Found product: {:?}", product);
    } else {
        println!("Product not found");
    }
    
    // Обновление товара
    match store.update_product("Laptop", Some(900.0), None) {
        Ok(_) => println!("Product updated successfully"),
        Err(e) => println!("Error updating product: {}", e),
    }
    
    // Проверка, что товар обновлен
    if let Some(product) = store.find_product("Laptop") {
        println!("Found product: {:?}", product);
    } else {
        println!("Product not found");
    }
    
    // Удаление товара
    match store.remove_product("Laptop") {
        Ok(_) => println!("Product removed successfully"),
        Err(e) => println!("Error removing product: {}", e),
    }
    // Проверка, что товар удален
    if let Some(product) = store.find_product("Laptop") {
        println!("Found product: {:?}", product);
    } else {
        println!("Product not found");
    }
}