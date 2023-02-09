mod todo;

struct Todo {
    id: i32,
    content: String,
    checked: bool,
}

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    match args[1].as_str() {
        "add" | "a" => println!("Added \"{}\" with id \"{}\"", &args[2], 1),
        "delete" | "d" => println!("Deleted \"{}\" with id \"{}\"", "placeholder", 1),
        "check" | "c" => println!("Deleted \"{}\" with id \"{}\"", "placeholder", 1),
        "uncheck" | "u" => println!("Unchecked \"{}\" with id \"{}\"", "placeholder", 1),
        "edit" | "e" => println!(
            "Changed from \"{}\" to \"{}\" with id \"{}\"",
            "placeholder 1", "placeholder 2", 1
        ),
        "list" | "l" | _ => {
            for item in [
                Todo {
                    id: 1,
                    content: String::from("Do laundry"),
                    checked: false,
                },
                Todo {
                    id: 2,
                    content: String::from("Listen to In Rainbows by Radiohead"),
                    checked: true,
                },
                Todo {
                    id: 3,
                    content: String::from("Do dishes"),
                    checked: false,
                },
            ] {
                println!(
                    "{}. [{}] {}",
                    item.id,
                    if item.checked { "x" } else { " " },
                    item.content,
                )
            }
        }
    }
}
