use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Mode {
    New,
    Complete,
    Editing,
}

#[derive(PartialEq, Clone)]
pub struct Todo {
    label: String,
    mode: Mode,
}

impl Todo {
    pub fn new(label: &str, mode: Mode) -> Self {
        Self {
            label: label.to_string(),
            mode,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TodoListProps {
    pub todos: Vec<Todo>,
}

#[function_component(TodoList)]
pub fn todo_list(props: &TodoListProps) -> Html {
    html! (
        <ul class="todo-list">
        { for props.todos.iter().map(|t| {
            html! {
                <TodoItem todo={t.clone()} />
            }
        })}
    </ul>
    )
}

#[derive(Properties, PartialEq)]
struct TodoItemProps {
    todo: Todo,
}

#[function_component(TodoItem)]
fn todo_item(props: &TodoItemProps) -> Html {
    let class_mode = match props.todo.mode {
        Mode::New => None,
        Mode::Complete => Some("Completed"),
        Mode::Editing => Some("editing"),
    };

    html! (
    <li class={classes!(class_mode)}>
        <div class="view">
            <input type="checkbox" class="toggle"
                checked={matches!(props.todo.mode, Mode::Complete)}/>
            <label>{&props.todo.label}</label>
            <button class="destroy" />
        </div>
        <input class="edit" type="text" value={props.todo.label.clone()}
            hidden={!matches!(props.todo.mode, Mode::Editing)}/>
    </li>
    )
}
