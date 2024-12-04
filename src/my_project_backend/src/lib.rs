use std::cell::RefCell;

use crate::config::Config;
use crate::blog::Blog;

mod blog;
mod config;

thread_local! {
    static BLOGS: RefCell<Vec<Blog>> = RefCell::new(Vec::new());
    static CONFIG: RefCell<Config> = RefCell::new(Config::new());
}

#[ic_cdk::update]
fn edit_config(new_config: Config) {
    CONFIG.with(|config| *config.borrow_mut() = new_config);
}

#[ic_cdk::update]
fn add_blog(title: String, content: String, tags: Vec<String>) -> Result<Blog, String> {
    let config = CONFIG.with(|config| config.borrow().clone());

    if title.len() > config.max_title_len as usize {
        Err(String::from("Title is too long"))
    }
    else if content.len() > config.max_content_len as usize {
        Err(String::from("Content is too long"))
    }
    else if tags.len() > config.max_tags_count as usize {
        Err(String::from("Too many tags"))
    }
    else if tags.iter().any(|tag| !config.tags.contains(tag)) {
        Err(String::from("Tag doesn't exist"))
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
