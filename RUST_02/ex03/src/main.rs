#[derive(PartialEq, PartialOrd, Debug)]
pub struct MyType {
    id: u32,
}

impl MyType {
    pub fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }

    pub fn lt(&self, other: &Self) -> bool {
        return self.id < other.id;
    }

    pub fn le(&self, other: &Self) -> bool {
        return self.id <= other.id;
    }

    pub fn gt(&self, other: &Self) -> bool {
        return self.id > other.id;
    }

    pub fn ge(&self, other: &Self) -> bool {
        return self.id >= other.id;
    }

    pub fn default() -> Self {
        let instance = Self { id: 0 };

        return instance;
    }

    pub fn clone(&self) -> Self {
        let instance = Self { id: self.id };

        return instance;
    }
}

fn main() {
    let instance = MyType::default();

    let other_instance = instance.clone();

    println!("the default value of MyType is {instance:?}");
    println!("the clone of `instance` is {other_instance:#?}");
    assert_eq!(
        instance,
        MyType::default(),
        "the default value isn't always the same :/"
    );
    assert_eq!(instance, other_instance, "the clone isn't the same :/");
    assert!(
        instance >= other_instance && instance <= other_instance,
        "why would the clone be less or greater than the original?",
    );
}
