use std::cell::RefCell;

use crate::blog::Blog;

mod blog;

thread_local! {
    static BLOGS: RefCell<Vec<Blog>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn add_blog(title: String, content: String, tags: Vec<String>) -> Result<(), String> {
    if title.len() > 250 {
        return Err(String::from("Title is too long"));
    }
    BLOGS.with(|blogs|
         blogs.borrow_mut().push(
            Blog::new(title, content, tags)
        ));
    Ok(())
}

#[ic_cdk::query]
fn get_blogs() -> Vec<Blog> {
    BLOGS.with(|blogs| blogs.borrow().clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
