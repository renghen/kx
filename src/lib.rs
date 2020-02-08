use std::cell::RefCell;
use std::error::Error;

#[derive(Debug)]
pub struct KubeConfig<'a> {
    contents: RefCell<Vec<&'a str>>,
}

impl<'a> KubeConfig<'a> {
    pub fn load(contents: Vec<&'a str>) -> Result<KubeConfig, Box<dyn Error>> {
        Ok(KubeConfig {
            contents: RefCell::new(contents),
        })
    }

    pub fn list_contexts(&self) -> String {
        self.get_contexts()
            .unwrap()
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn get_config(&self) -> String {
        self.contents
            .borrow()
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn set_current_context(&'a self, new_context: &'a str) -> Result<(), &'static str> {
        let mut index = 0;
        let mut contents = self.contents.borrow_mut();

        {
            let mut iter = contents.iter();

            while let Some(line) = iter.next() {
                if parser::match_literal(line, "current-context: ").is_some() {
                    break;
                }
                index = index + 1;
            }
        }

        contents.push(new_context);
        contents.swap_remove(index);

        Ok(())
    }

    fn get_contexts(&self) -> Result<Vec<&str>, &'static str> {
        let mut contexts = Vec::<&str>::new();
        let contents = self.contents.borrow();
        let mut input = contents.iter().peekable();

        while let Some(line) = input.next() {
            if parser::match_literal(line, "contexts:").is_some() {
                // unwrap
                while parser::is_in_mapping(input.peek().unwrap()).is_ok() {
                    if let Some(line) = input.next() {
                        if let Some(name) = parser::match_literal(line, "  name: ") {
                            contexts.push(name);
                        }
                    }
                }

                break;
            }
        }

        if contexts.len() == 0 {
            return Err("Cannot get contexts!");
        }

        Ok(contexts)
    }
}

mod parser;
