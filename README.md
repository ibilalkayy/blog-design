# **Blog Design**

Welcome to the **Blog Design** repository! This project aims to create a full-featured blogging application using the Rust programming language, leveraging the Actix framework for backend development, Diesel ORM with PostgreSQL for database management, and HTMX with Tailwind CSS for frontend development.

## Features

- **User Authentication**
  - Create an account
  - Login/Logout
  - Update your profile
  
- **Blog Management**
  - Create a new blog post
  - Edit and update existing blog posts
  - Delete blog posts
  
- **Commenting System**
  - Add comments to blog posts
  - Edit and delete your comments

- **Responsive Design**
  - Styled with Tailwind CSS
  - Interactive elements powered by HTMX

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (Comes with Rust)
- [PostgreSQL](https://www.postgresql.org/download/)
- [Diesel CLI](https://diesel.rs/guides/getting-started) (For database migrations)
- [Node.js](https://nodejs.org/) (For Tailwind CSS)
- [pnpm](https://pnpm.io/installation) (Package manager for frontend dependencies)

### Project Structure

- **src/**: Contains the Rust source code for the backend.
- **migrations/**: Database migrations managed by Diesel.
- **static/**: Static assets such as CSS files.
- **templates/**: HTML templates rendered by the Actix framework.

### Contributing

We welcome contributions! Please fork the repository and create a pull request with your changes. Ensure your code is well-tested and follows Rust's coding standards.

### License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
