use sqlx::SqlitePool;

pub struct _Todo {
    id: i32,
    content: String,
    checked: bool,
}

impl _Todo {
    async fn _get_all(_pool: &SqlitePool) -> _Todo {
        _Todo {
            id: 1,
            checked: true,
            content: String::from(""),
        }
    }
    async fn _add(_pool: &SqlitePool, _new_todo: _Todo) -> _Todo {
        _Todo {
            id: 1,
            checked: true,
            content: String::from(""),
        }
    }
    async fn _update(_pool: &SqlitePool, _id: i32, _updated_todo: _Todo) -> _Todo {
        _Todo {
            id: 1,
            checked: true,
            content: String::from(""),
        }
    }
    async fn _delete(_pool: &SqlitePool, _id: i32) -> _Todo {
        _Todo {
            id: 1,
            checked: true,
            content: String::from(""),
        }
    }
}
