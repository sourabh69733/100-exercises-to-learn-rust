// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct  Order {
    pub product_name: String,
    pub quantity: i32,
    pub unit_price: i32,
}

impl Order {
    fn is_product_name_valid(product_name: &String) -> bool {
        let product_len = product_name.as_bytes().len();

        if product_len > 300 || product_len <= 0 {
            panic!("Product name is either empty or its length is greater than 300")
        }

        return true
    }

    fn is_quantity_valid(quantity: &i32) -> bool {
        if *quantity <= 0 {
            panic!("Quantity must be greater than 0")
        }

        return true
    }

    fn is_unit_price_valid(unit_price: &i32) -> bool {
        if *unit_price <= 0 {
            panic!("Unit price must be greater than 0")
        }

        return true
    }

    pub fn new(product_name: String, quantity: i32, unit_price: i32) -> Self {
        Self::is_product_name_valid(&product_name);
        Self::is_quantity_valid(&quantity);
        Self::is_unit_price_valid(&unit_price);

        Self {
            product_name,
            quantity,
            unit_price
        }
    }

    pub fn unit_price(&self) -> &i32 {
        return &self.unit_price
    }

    pub fn product_name(&self) -> String {
        return self.product_name.clone()
    }

    pub fn quantity(&self) -> &i32 {
        return &self.quantity
    }

    pub fn set_product_name(&mut self, product_name: String) {
        Self::is_product_name_valid(&product_name);

        self.product_name = product_name;
    }

    pub fn set_quantity(&mut self, quantity: i32) {
        Self::is_quantity_valid(&quantity);

        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: i32) {
        Self::is_unit_price_valid(&unit_price);

        self.unit_price = unit_price;
    }

    pub fn total(&self) -> i32 {
        return self.unit_price * self.quantity;
    }
}