# Personnel Management System

A simple personnel management system built using Rust, Actix Web, and MongoDB. This application provides an API for managing users and employees, including functionalities for creating, reading, updating, and deleting records.

## Features

- User management
- Employee management
- RESTful API
- MongoDB for data storage

## Technologies Used

- Rust
- Actix Web
- MongoDB
- serde_json
- futures

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
- [MongoDB](https://www.mongodb.com/try/download/community) installed and running.

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/personnel.git
   cd personnel


API Endpoints
```
POST /users: Create a new user

GET /users: Retrieve all users

PUT /users/{id}: Update a user

DELETE /users/{id}: Delete a user

POST /employees: Create a new employee

GET /employees: Retrieve all employees

PUT /employees/{id}: Update an employee

DELETE /employees/{id}: Delete an employee

```
Architecture
src/
├── main.rs              // Entry point
├── config.rs            // Configuration management
├── db/
│   ├── mod.rs           // Database module
│   ├── connection.rs     // Database connection handling
│   ├── repositories/     // Repository pattern implementation
│   │   ├── mod.rs
│   │   ├── user_repo.rs
│   │   └── employee_repo.rs
├── handlers/            // Request handlers
│   ├── mod.rs
│   ├── user_handler.rs
│   └── employee_handler.rs
├── middleware/          // Custom middleware
│   ├── mod.rs
│   └── auth.rs          // Authentication middleware
├── models/              // Domain models
│   ├── mod.rs
│   ├── user.rs
│   └── employee.rs
├── routes/              // Route definitions
│   ├── mod.rs
│   └── user_routes.rs
│   └── employee_routes.rs
├── services/            // Business logic
│   ├── mod.rs
│   ├── user_service.rs
│   └── employee_service.rs
└── utils/               // Utility functions
    ├── mod.rs
    └── logger.rs        // Custom logging implementation




Contributing
Contributions are welcome! Please feel free to submit a pull request or open an issue.

License




