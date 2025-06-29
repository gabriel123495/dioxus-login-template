use dioxus::prelude::*;
use dioxus_desktop::launch;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
        rsx! {
        div {
            style: "
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                height: 100vh;
                gap: 20px;
            ",
            input {
                r#type: "text",
                style: "
                    background-color: #f5f5f5;
                    color: #222;
                    border: 1px solid #ccc;
                    border-radius: 6px;
                    padding: 10px;
                    width: 250px;
                ",
                placeholder: "digite um nome de usuario"
            }
            input {
                r#type: "email",
                style: "
                    background-color: #f5f5f5;
                    color: #222;
                    border: 1px solid #ccc;
                    border-radius: 6px;
                    padding: 10px;
                    width: 250px;
                ",
                placeholder: "digite o seu gmail"
            }
            input {
                r#type: "password",
                style: "
                    background-color: #f5f5f5;
                    color: #222;
                    border: 1px solid #ccc;
                    border-radius: 6px;
                    padding: 10px;
                    width: 250px;
                ",
                placeholder: "digite uma senha"
            }
            button {
                style: "
                    background-color: #2563eb;
                    padding: 13px 0;
                    color: white;
                    border: none;
                    border-radius: 6px;
                    width: 250px;
                    font-weight: bold;
                    cursor: pointer;
                ",
                "login"
            }
        }
    }
}