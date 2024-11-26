use std::cell::RefCell;

thread_local! {
    static CHAT: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn save_chat(input_chat: String) -> String {
    CHAT.with(|chat| {
        chat.borrow_mut().push(input_chat);
    });
    "Chat saved".to_string() // Return a string indicating success
}

#[ic_cdk::query]
fn get_chat() -> String {
    CHAT.with(|chat| {
        chat.borrow().join("\n") // Join the chat messages into a single string
    })
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}