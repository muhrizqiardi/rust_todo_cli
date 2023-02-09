mod todo;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args = std::env::args().collect::<Vec<String>>();
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(database_url) => database_url,
        Err(_) => String::from("sqlite:todo.db"),
    };
    let pool = SqlitePool::connect(&database_url).await?;

    match args[1].as_str() {
        "add" | "a" => {
            let result = sqlx::query!("INSERT INTO Todo (content) VALUES (?) RETURNING *", args[2])
                .fetch_all(&pool)
                .await?;

            println!(
                "Added \"{:?}\" with id \"{}\"",
                result[0].content, result[0].id
            )
        }
        "delete" | "d" => {
            println!("Deleted \"{}\" with id \"{}\"", "placeholder", 1)
        }
        "check" | "c" => println!("Deleted \"{}\" with id \"{}\"", "placeholder", 1),
        "uncheck" | "u" => println!("Unchecked \"{}\" with id \"{}\"", "placeholder", 1),
        "edit" | "e" => println!(
            "Changed from \"{}\" to \"{}\" with id \"{}\"",
            "placeholder 1", "placeholder 2", 1
        ),
        "list" | "l" | _ => {
            let result = sqlx::query!("SELECT * FROM Todo").fetch_all(&pool).await?;

            for item in result {
                println!(
                    "{}. [{}] {}",
                    item.id,
                    if item.checked { "x" } else { " " },
                    item.content,
                )
            }
        }
    }

    Ok(())
}
