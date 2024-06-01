#[derive(Default, Debug, Clone)]
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

    pub fn components(self) -> Vec<String> {
        self.components
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
}

impl From<String> for Path {
    fn from(path: String) -> Self {
        Self::new(&path)
    }
}
