# Comprehensive CRUD Operations with PostgreSQL in Rust Using Tokio-Postgres

`Prerequisites`:

- Knowledge of Rust and asynchronous programming.
- Installed [Tokio](https://crates.io/crates/tokio) and [tokio-postgres](https://crates.io/crates/tokio-postgres) libraries.
- A running PostgreSQL server with appropriate access.

## Step 1: Set up the required imports

Start by importing necessary modules and libraries.

```rs
use std::error::Error;
use tokio_postgres::{NoTls, Row};
```

## Step 2: Define the CRUD functions

### 2.1 Create - Insert a new user into the database.

```rs
async fn create_user(client: &tokio_postgres::Client, name: &str, age: i32) -> Result<(), Box<dyn Error>> {
    client.execute(
        "insert into users (name, age) values ($1, $2)",
        &[&name, &age]
    ).await?;
    Ok(())
}
```

### 2.2 Read All - Retrieve all users from the database.

```rs
async fn read_users(client: &tokio_postgres::Client) -> Result<Vec<Row>, Box<dyn Error>> {
    let rows = client.query("select id, name, age from users", &[]).await?;
    Ok(rows)
}
```

### 2.3 Read Single User by ID - Retrieve a specific user by their ID.

```rs
async fn read_user_by_id(client: &tokio_postgres::Client, id: i32) -> Result<Option<(i32, String, i32)>, Box<dyn Error>> {
    let rows = client.query("select id, name, age from users where id = $1", &[&id]).await?;
    if rows.is_empty() {
        Ok(None)
    } else {
        let row = &rows[0];
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let age: i32 = row.get(2);
        Ok(Some((id, name, age)))
    }
}
```

### 2.4 Update - Modify a user's age based on their ID.

```rs
async fn update_user_age(client: &tokio_postgres::Client, id: i32, new_age: i32) -> Result<(), Box<dyn Error>> {
    client.execute("update users set age = $1 where id = $2", &[&new_age, &id]).await?;
    Ok(())
}
```

### 2.5 Delete - Remove a user by their ID.

```rs
async fn delete_user(client: &tokio_postgres::Client, id: i32) -> Result<(), Box<dyn Error>> {
    client.execute("delete from users where id = $1", &[&id]).await?;
    Ok(())
}
```

## Step 3: Connect to the database and demonstrate the CRUD operations

Your `main` function will serve as a demonstration ground for all the CRUD operations:

```rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ...
}
```

### 3.1 Connect to the PostgreSQL database using the provided connection string.

```rs
let (client, connection) = tokio_postgres::connect(
    "host=localhost user=postgres dbname=database_name password=secret",
    NoTls
).await?;
```

### 3.2 Spawn the database connection to let it run in the background.

```rs
tokio::spawn(async move {
    if let Err(e) = connection.await {
        eprint!("connection error: {}", e);
    }
});
```

### 3.3 Demonstrate the CRUD operations:

```rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the database
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres dbname=database_name password=secret",
        NoTls,
    )
    .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprint!("connection error: {}", e);
        }
    });

    // CREATE
    create_user(&client, "Htet Lin Maung", 27).await?;

    // READ
    let users = read_users(&client).await?;
    for user in &users {
        let id: i32 = user.get(0);
        let name: String = user.get(1);
        let age: i32 = user.get(2);
        println!("id: {}, name: {}, age: {}", id, name, age);
    }

    // FETCH BY ID (Fetch Alice by her id)
    if let Some((id, name, age)) = read_user_by_id(&client, 1).await? {
        println!("Fetched by ID -> id: {}, name: {}, age: {}", id, name, age);
    } else {
        println!("User not found by given ID");
    }

    // UPDATE (Update Alice's age to 31)
    update_user_age(&client, 1, 31).await?;

    // DELETE (Delete Alice by id)
    delete_user(&client, 1).await?;

    Ok(())
}
```
