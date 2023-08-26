use std::error::Error;

use tokio_postgres::{NoTls, Row};

// CREATE: Insert a new user into the database
async fn create_user(
    client: &tokio_postgres::Client,
    name: &str,
    age: i32,
) -> Result<(), Box<dyn Error>> {
    client
        .execute(
            "insert into users (name, age) values ($1, $2)",
            &[&name, &age],
        )
        .await?;
    Ok(())
}

// READ: Get all users from the database
async fn read_users(client: &tokio_postgres::Client) -> Result<Vec<Row>, Box<dyn Error>> {
    let rows = client.query("select id, name, age from users", &[]).await?;
    Ok(rows)
}

// Retrieve a user by their ID
async fn read_user_by_id(
    client: &tokio_postgres::Client,
    id: i32,
) -> Result<Option<(i32, String, i32)>, Box<dyn Error>> {
    let rows = client
        .query("select id, name, age from users where id = $1", &[&id])
        .await?;
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

// UPDATE: Update a user's age by id
async fn update_user_age(
    client: &tokio_postgres::Client,
    id: i32,
    new_age: i32,
) -> Result<(), Box<dyn Error>> {
    client
        .execute("update users set age = $1 where id = $2", &[&new_age, &id])
        .await?;
    Ok(())
}

// DELETE: Remove a user by id
async fn delete_user(client: &tokio_postgres::Client, id: i32) -> Result<(), Box<dyn Error>> {
    client
        .execute("delete from users where id = $1", &[&id])
        .await?;
    Ok(())
}

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
