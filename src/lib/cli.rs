

use crate::lib::core::Container;

#[derive(Debug)]
pub struct Arguments<'a> {
    pub image_name: String,
    pub image_dir: String,
    pub container_dir: String,
    pub command: Vec<&'a str>,
}

impl<'a> Arguments<'a> {
    pub fn new(
        image_name: String,
        image_dir: String,
        container_dir: String,
        command: Vec<&'a str>,
    ) -> Arguments<'a> {
        Arguments {
            image_name,
            image_dir,
            container_dir,
            command,
        }
    }
}

pub fn run(args: Arguments) {
    let container = Container::new(args);
    println!("Creating new container:\n{:?}", container);
    container.run()
}
