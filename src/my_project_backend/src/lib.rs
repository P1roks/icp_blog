use std::cell::RefCell;

use crate::blog::Blog;

mod blog;

thread_local! {
    static BLOGS: RefCell<Vec<Blog>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn add_blog(title: String, date: u32,
     content: String, tags: Vec<String>) {
    BLOGS.with(|blogs|
         blogs.borrow_mut().push(
            Blog::new(title, date, content, tags)
        ));
}

#[ic_cdk::query]
fn get_blogs() -> Vec<Blog> {
    BLOGS.with(|blogs| blogs.borrow().clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
