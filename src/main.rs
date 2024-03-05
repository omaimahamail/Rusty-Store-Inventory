use std::io;

struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    fn delete_product(&mut self, index: usize) -> Result<(), &'static str> {
        if index < self.products.len() {
            self.products.remove(index);
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    fn generate_report(&self) {
        println!("\n\tRust Store Inventory Report:\n");
        println!("{:-<111}", "");
        println!("| {:5} | {:<15} | {:<45} | {:<15} | {:<15} |", "S.No", "Name", "Description", "Price", "Quantity");
        println!("{:-<111}", "");

        for (index, product) in self.products.iter().enumerate() {
            println!("| {:<5} | {:<15} | {:<45} | Rs {:<12} | {:<15} |", index + 1, product.name, product.description, product.price, product.quantity);
        }
        println!("{:-<111}", "");
    }
}

struct User {
    username: String,
    password: String,
}

impl User {
    fn new(username: String, password: String) -> User {
        User { username, password }
    }

    fn authenticate(&self, username: &str, password: &str) -> bool {
        self.username == username && self.password == password
    }
}

struct Menu;

impl Menu {
    fn display() {
        println!("\n\tRusty Store Inventory Management System\n");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. Generate Inventory Report");
        println!("5. Exit");
        println!("\nPlease enter your choice:");
    }

    fn process_choice(&mut self, choice: u32, inventory: &mut Inventory) {
        match choice {
            1 => self.add_product(inventory),
            2 => self.edit_product(inventory),
            3 => self.delete_product(inventory),
            4 => inventory.generate_report(),
            5 => {
                println!("Exiting...");
                std::process::exit(0);
            }
            _ => println!("Invalid choice. Please enter a valid option."),
        }
    }

    fn add_product(&mut self, inventory: &mut Inventory) {
        println!("Enter product details:");

        println!("Name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim().to_string();

        println!("Description:");
        let mut description = String::new();
        io::stdin().read_line(&mut description).expect("Failed to read line");
        let description = description.trim().to_string();

        println!("Price:");
        let mut price = String::new();
        io::stdin().read_line(&mut price).expect("Failed to read line");
        let price: f64 = price.trim().parse().expect("Please enter a valid price");

        println!("Quantity:");
        let mut quantity = String::new();
        io::stdin().read_line(&mut quantity).expect("Failed to read line");
        let quantity: u32 = quantity.trim().parse().expect("Please enter a valid quantity");

        let new_product = Product { name, description, price, quantity };
        inventory.add_product(new_product);
        println!("Product added successfully!");
    }

    fn edit_product(&mut self, inventory: &mut Inventory) {
        println!("Enter the index of the product you want to edit:");
        let mut index_input = String::new();
        io::stdin().read_line(&mut index_input).expect("Failed to read line");
        let index: usize = index_input.trim().parse().expect("Please enter a valid index");

        if let Some(product) = inventory.products.get_mut(index) {
            println!("Enter the field you want to edit (name, description, price, quantity):");
            let mut field = String::new();
            io::stdin().read_line(&mut field).expect("Failed to read line");
            let field = field.trim();

            match field {
                "name" => {
                    println!("Enter new name:");
                    let mut name = String::new();
                    io::stdin().read_line(&mut name).expect("Failed to read line");
                    product.name = name.trim().to_string();
                }
                "description" => {
                    println!("Enter new description:");
                    let mut description = String::new();
                    io::stdin().read_line(&mut description).expect("Failed to read line");
                    product.description = description.trim().to_string();
                }
                "price" => {
                    println!("Enter new price:");
                    let mut price = String::new();
                    io::stdin().read_line(&mut price).expect("Failed to read line");
                    product.price = price.trim().parse().expect("Please enter a valid price");
                }
                "quantity" => {
                    println!("Enter new quantity:");
                    let mut quantity = String::new();
                    io::stdin().read_line(&mut quantity).expect("Failed to read line");
                    product.quantity = quantity.trim().parse().expect("Please enter a valid quantity");
                }
                _ => println!("Invalid field. Please enter a valid field (name, description, price, quantity)."),
            }
            println!("Product edited successfully!");
        } else {
            println!("Invalid index. No product found at index {}.", index);
        }
    }

    fn delete_product(&mut self, inventory: &mut Inventory) {
        println!("Enter the index of the product you want to delete:");
        let mut index_input = String::new();
        io::stdin().read_line(&mut index_input).expect("Failed to read line");
        let index: usize = index_input.trim().parse().expect("Please enter a valid index");

        match inventory.delete_product(index) {
            Ok(_) => println!("Product deleted successfully!"),
            Err(err) => println!("{}", err),
        }
    }
}

fn main() {
    let mut inventory = Inventory { products: Vec::new() };
    let user = User::new(String::from("admin"), String::from("password"));

    println!("\n\tWelcome to My Rusty Store Inventory Management System\n");

    println!("Please enter your username:");
    let mut input_username = String::new();
    io::stdin().read_line(&mut input_username).expect("Failed to read line");
    let input_username = input_username.trim();

    println!("Please enter your password:");
    let mut input_password = String::new();
    io::stdin().read_line(&mut input_password).expect("Failed to read line");
    let input_password = input_password.trim();

    if user.authenticate(input_username, input_password) {
        println!("\nAuthentication successful!");

        let mut menu = Menu;

        loop {
            Menu::display();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read line");
            let choice: u32 = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            menu.process_choice(choice, &mut inventory);
        }
    } else {
        println!("\nAuthentication failed. Access denied!");
    }
}