
/**
 * The snippet struct that has uses multiple tags, to order the snippets
 */
#[derive(Debug)]
struct Snippet {
    name: String;
    tags: Vec<String>;
}

impl Snippet {

    pub fn new(name: String, tags: Vec<String>) {
        Snippet{name, tags}
    }

}





