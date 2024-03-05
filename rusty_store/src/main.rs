use std::io;
use prettytable::{Table, Row, Cell};

trait Inventory {
    fn add_item(inventory_vec: &mut Vec<Product>);
    fn edit_item(inventory_vec: &mut Vec<Product>);
    fn delete_item(inventory_vec: &mut Vec<Product>);
}

#[derive(Debug, PartialEq)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

struct User {
    username: String,
    password: String,
}


impl Inventory for Product {
    fn add_item(inventory_vec: &mut Vec<Product>) {
        let name = user_input("Enter the product name: ");
        let description = user_input("Enter the product description: ");
        let price: f64 = user_input("Enter the product price: ").trim().parse().expect("\nNot a valid price\n");
        let quantity: u32 = user_input("Enter the product quantity: ").trim().parse().expect("\nNot a valid quantity\n");
        let product = Product { name: name, description: description, price: price, quantity: quantity };
        inventory_vec.push(product);
    }

    fn edit_item(inventory_vec: &mut Vec<Product>) {
        let search_name = user_input("Enter product name you want to edit: ");
        let product = inventory_vec.iter_mut().find(|p| p.name == search_name);
        match product {
            Some(p) => {
                let choice: u8 = user_input("What feature do you want to edit: \n1. Product Name \n2. Description \n3. Price \n4. Quantity\n\nChoice: ")
                .trim().parse().expect("Invalid Input");
                match choice {
                    1 => {
                        let new_name = user_input("Enter new product name: ");
                        p.name = new_name;
                    }
                    2 => {
                        let new_description = user_input("Enter new description: ");
                        p.description = new_description;
                    }
                    3 => {
                        let new_price = user_input("Enter new price: ");
                        if let Ok(new_price) = new_price.trim().parse::<f64>() {
                            p.price = new_price;
                        } else {
                            println!("\nInvalid price format!\n");
                        }
                    }
                    4 => {
                        let new_quantity = user_input("Enter new quantity: ");
                        if let Ok(new_quantity) = new_quantity.trim().parse::<u32>() {
                            p.quantity = new_quantity;
                        } else {
                            println!("\nInvalid quantity format!\n");
                        }
                    }
                    _ => println!("\nInvalid choice!\n"),
                }
            }
            None => println!("\nProduct not found!\n"),
        }
    }
    

    fn delete_item(inventory_vec: &mut Vec<Product>) {
        let search_name = user_input("Enter product name you want to delete: ");
        let index = inventory_vec.iter().position(|p| p.name == search_name);
        match index {
            Some(i) => {
                inventory_vec.remove(i);
                println!("\nProduct '{search_name}' deleted successfully!\n");
            }
            None => println!("\nProduct '{search_name}' not found!\n"),
        }
    }

}

fn user_authentication(user: User) -> bool {
    let username = user_input("Enter username: ");
    let password = user_input("Enter password: ");

    if username.trim() == user.username && password.trim() == user.password {
        println!("\n\n\t\tWelcome to the Rusty Store\n\n");
        true
    }else{
        println!("\n\n\tWrong Username or password!!!");
        false
    }
}

fn generate_report(inventory_vec: &Vec<Product>) {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Product Name"),
        Cell::new("Description"),
        Cell::new("Price"),
        Cell::new("Quantity"),
    ]));

    for product in inventory_vec {
        table.add_row(Row::new(vec![
            Cell::new(&product.name),
            Cell::new(&product.description),
            Cell::new(&product.price.to_string()),
            Cell::new(&product.quantity.to_string()),
        ]));
    }

    table.printstd();
}

fn user_input(prompt: &str) -> String {
    println!("\n{prompt}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("\nNot a valid string\n");
    input
}

fn main() {
    let user = User {
        username: String::from("Manager"),
        password: String::from("Password123"),
    };

    let mut inventory_vec: Vec<Product> = Vec::new();

    if user_authentication(user) {
        loop {
            let user_choice: u8 = user_input("What action do you wish to perform: \n1. Add product \n2. Edit product \n3. Delete Product \n4. Generate Report \n5. Exit\n\nChoice: ")
            .trim().parse().expect("Invalid Input");
            match user_choice {
                1 => <Product as Inventory>::add_item(&mut inventory_vec),
                2 => <Product as Inventory>::edit_item(&mut inventory_vec),
                3 => <Product as Inventory>::delete_item(&mut inventory_vec),
                4 => generate_report(&inventory_vec),
                5 => break,
                _ => println!("\nInvalid choice\n"),
            }
        }
    }
    println!("\n\nThank you for using our rusty store.")
}