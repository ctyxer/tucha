#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    path: String,
    components: Vec<String>,
}

impl Path {
    pub fn new(path: &str) -> Self {
        let path = path.to_string();
        let components = path
            .split("/")
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        Self { path, components }
    }

    pub fn components(&self) -> Vec<String> {
        self.components.clone()
    }

    pub fn name(&self) -> Option<&str> {
        self.components.last().map(|v| v.as_str())
    }

    pub fn parent(&self) -> Self {
        let mut parent = self.clone();
        parent.components.pop();
        parent.path = parent.components.join("/");
        parent
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn join(&self, path: &str) -> Self {
        Self::new(&format!("{}{}", self.path, path))
    }

    pub fn push(&mut self, path: &str) {
        self.path.push_str(&format!("{}/", path));
        self.components = Self::new(&self.path).components();
    }

    pub fn pop(&mut self) {
        self.components.pop();
        match self.components.len() > 0 {
            true => self.path = format!("/{}/", self.components.join("/")),
            false => self.path = "/".to_string(),
        }
    }
}

impl From<String> for Path {
    fn from(path: String) -> Self {
        Self::new(&path)
    }
}

impl Default for Path {
    fn default() -> Self {
        Self {
            path: String::from("/"),
            components: Vec::new(),
        }
    }
}
