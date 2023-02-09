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
            let query_result = sqlx::query!("DELETE FROM Todo WHERE id = ? RETURNING *", args[2])
                .fetch_all(&pool)
                .await?;

            println!(
                "Deleted \"{}\" with id \"{}\"",
                query_result[0].content, query_result[0].id
            )
        }
        "check" | "c" => println!("Deleted \"{}\" with id \"{}\"", "placeholder", 1),
        "uncheck" | "u" => println!("Unchecked \"{}\" with id \"{}\"", "placeholder", 1),
        "edit" | "e" => {
            let query_result = sqlx::query!(
                "UPDATE Todo
                SET content = ?
                WHERE id = ?
                RETURNING *",
                args[3],
                args[2],
            )
            .fetch_all(&pool)
            .await?;

            println!(
                "Changed from \"{:?}\" to \"{:?}\" with id \"{}\"",
                args[3], query_result[0].content, args[2]
            );
        }
        "list" | "l" | _ => {
            let query_result = sqlx::query!("SELECT * FROM Todo").fetch_all(&pool).await?;

            if query_result.len() == 0 {
                println!("(empty)")
            };

            for item in query_result {
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
