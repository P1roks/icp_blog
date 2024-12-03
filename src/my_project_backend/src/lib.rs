use std::cell::RefCell;

use crate::blog::Blog;

mod blog;

thread_local! {
    static BLOGS: RefCell<Vec<Blog>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn add_blog(title: String, content: String, tags: Vec<String>) -> Result<Blog, String> {
    if title.len() > 250 {
        Err(String::from("Title is too long"))
    }
    else if content.len() > 500 {
        Err(String::from("Content is too long"))
    }
    else if tags.len() > 3 {
        Err(String::from("Too many tags"))
    }
    else {
        BLOGS.with(|blogs| {
            blogs.borrow_mut().push(
                Blog::new(title, content, tags)
            );
    
            match blogs.borrow().last() {
                Some(val) => Ok(val.clone()),
                None => Err(String::from("Vec should not be empty")),
            }
        })
    }
}

#[ic_cdk::query]
fn get_blogs() -> Vec<Blog> {
    BLOGS.with(|blogs| blogs.borrow().clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
