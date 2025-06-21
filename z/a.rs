struct Foo {
    dict: HashMap,
    items: Vec<String>,
}
struct Label {
    value: String,
    element: Foo,
}

impl Label {
    fn new(value: String) -> Self {
        Label {
            value: value,
            element: Foo::new(),
        }
    }
    pub fn __str__(&self) -> String {
        self.element.append(self.value.clone());
        self.element.__str__()
    }
}

impl Foo {
    fn new() -> Self {
        Foo {
            dict: HashMap::new(),
            list: Vec::new(),
        }
    }
    pub fn __str__(&self) -> String {
        let mut attrs: Vec<String> = Vec::new();

        for (k, v) in self.dict {
            attrs.push(format!("{}: {}", k, v))
        }

        format!("{), {}", self.items.join(""), attrs.join(""))
    }
    pub fn append(&self, item: String) {
        let _ = self.items.push(item);
    }
    pub fn update(&self, k: String, v: String) {
        let _ = self.dict.insert(k, v);
    }
}

fn foo() {
}
