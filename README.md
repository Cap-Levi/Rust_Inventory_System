# Rusty Store

![image_LoVP97iQ_1709656484993_raw](https://github.com/Cap-Levi/rusty-store/assets/76009148/a58bfced-ab16-459f-b087-47bb166516ba)

## Individual

Fazeel Azam, a sixth-semester BS Cybersecurity student at COMSATS, is deeply interested in web3 technologies, blockchain, and smart contracts. He actively participates in Capture The Flag (CTF) competitions, including the Ignite Hackathon and Pakcrypt. When not immersed in cybersecurity, Fazeel enjoys gaming.

## Description

"Rusty Store" is a terminal-based inventory management system developed in Rust. It offers a more efficient alternative to other programming languages. The project aims to provide a user-friendly interface for managing inventory, making it suitable for small retail stores. Users can add, remove, and update product information such as name, description, quantity, and price. The system ensures data integrity and security, crucial for inventory management. The terminal interface allows for easy navigation and quick access to various functions, enhancing user experience. Rusty Store's efficient design and user-friendly interface make it a reliable choice for small businesses looking to streamline their inventory management processes.

## Vision

The vision of "Rusty Store" is to inspire a shift towards more secure programming languages with fewer bugs and greater efficiency. By showcasing the advantages of Rust in developing robust and efficient software, the project aims to encourage developers and businesses to adopt Rust for their projects. This shift can lead to a significant impact, improving the overall security and performance of software applications. Additionally, by promoting Rust's capabilities through practical applications like inventory management, the project aims to create a ripple effect, encouraging more developers to explore and adopt Rust for a wide range of projects.

## Project Roadmap / Future Plans

**Software Development Plan for "Rusty Store"**

1. **Define Data Structures**: Define structs for products (name, description, quantity, price) and error types (ProductNotFound, InvalidInput).

2. **Implement CRUD Functions**: Implement functions to Create, Read, Update, and Delete products. Ensure error handling for ProductNotFound and InvalidInput.

3. **Add Authentication**: Implement a basic authentication system using username and password stored securely. Use the `verify_password` method to authenticate users.

4. **Develop Reporting Functionality**: Create functions to generate reports such as total sales, current inventory status, and popular products. Ensure the reports are clear and easy to understand.

5. **Test and Refine**: Test the application extensively, ensuring all functions work as expected and errors are handled gracefully. Refine the user interface for better usability.

6. **Deployment**: Deploy the application to a secure server, ensuring data integrity and security. Provide documentation for users on how to use the application.

## Tech We Use

Rust

## Setup Environment

### Overview

Rusty Store is a terminal-based inventory management system developed in Rust. It offers a more efficient alternative to other programming languages, making it suitable for small retail stores.

### Installation

Clone the repository:

`git clone https://github.com/Cap-Levi/rusty-store.git`

Navigate to the project directory:

`cd rusty-store`

Build the project using Cargo:

`cargo build`

Run the executable:

`cargo run`

### Usage

- Authentication

`username: Manager`
`password: Password123`

- Add a new product: **add_product**
- Update a product: **edit_product** (small modules in it as follows)
    * edit_name
    * edit_description
    * edit_price
    * edit_quantity                  
  
- Remove a product: **delete_product**
- View all products: **generate_report**

### Contributing

1. Fork the repository
2. Create a new branch (`git checkout -b feature/your-feature`)
3. Make your changes
4. Commit your changes (`git commit -am 'Add new feature'`)
5. Push to the branch (`git push origin feature/your-feature`)
6. Create a new Pull Request

### License
This project is licensed under the MIT License - see the LICENSE file for details.
