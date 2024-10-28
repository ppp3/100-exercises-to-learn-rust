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
pub struct Order
{
    product_name: String,
    quantity: u32,
    unit_price:u32
}

impl Order {
    pub fn set_unit_price(&mut self, p0: u32) {
        self.unit_price = p0;
    }
}

impl Order {
    pub fn set_quantity(&mut self, p0: u32) {
        self.quantity = p0;
    }
}

impl Order {
    pub fn set_product_name(&mut self, p0: String) {
        self.product_name = p0;
    }
}

impl Order {
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }
}

impl Order {
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
}

impl Order {
    pub fn product_name(&self) -> String {
        self.product_name.clone()
    }
}

impl Order
{
    pub fn new(name:String, quantity:u32, unit_price:u32 )->Self
    {
        Self::validate_name(&name);
        Self::validate_quantity(quantity);
        Self::validate_product_price(unit_price);
        Self{product_name:name, quantity, unit_price}
    }

    fn validate_name(name: &String)
    {
        if name.is_empty() {
            panic!("Name cannot be empty");
        }
        if name.len() > 300 {
            panic!("Name cannot be longer than 300 bytes");
        }
    }

    fn validate_quantity(quantity: u32)
    {
        if quantity <=0 {
        panic!("Name cannot be longer than 300 bytes");
    }
    }

    fn validate_product_price(price: u32)
    {
        if price <=0 {
            panic!("Name cannot be longer than 300 bytes");
        }
    }
    pub fn total(&self)-> u32
    {
        self.quantity*self.unit_price
    }


}