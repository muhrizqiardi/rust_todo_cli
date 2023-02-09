mod todo;

struct Todo {
    id: i32,
    content: String,
    checked: bool,
}

#[async_std::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args[1] == "list" || args[1] == "l" {
        let _placeholder_list = [
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
        ];

        for item in _placeholder_list {
            println!(
                "{}. [{}] {}",
                item.id,
                if item.checked { "x" } else { " " },
                item.content,
            )
        }
    }

    if args[1] == "add" || args[1] == "a" {
        return println!("Added \"{}\" with id \"{}\"", &args[2], 1);
    }

    if args[1] == "delete" || args[1] == "d" {
        return println!("Deleted \"{}\" with id \"{}\"", "placeholder", 1);
    }

    if args[1] == "check" || args[1] == "c" {
        return println!("Checked \"{}\" with id \"{}\"", "placeholder", 1);
    }

    if args[1] == "uncheck" || args[1] == "u" {
        return println!("Unchecked \"{}\" with id \"{}\"", "placeholder", 1);
    }

    if args[1] == "edit" || args[1] == "e" {
        return println!(
            "Changed from \"{}\" to \"{}\" with id \"{}\"",
            "placeholder 1", "placeholder 2", 1
        );
    }
}
