struct PuppyDog {
    pub name: String,
    pub age: usize,
}

#[allow(dead_code)]
struct LoopyDog {
    pub name: String,
    pub age: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Dog {
    pub name: String,
    pub age: usize,
    pub r#type: String,
}

impl TryFrom<PuppyDog> for Dog {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: PuppyDog) -> Result<Self, Self::Error> {
        let dog = Dog {
            name: value.name,
            age: value.age,
            r#type: String::from("Loopy"),
        };
        Ok(dog)
    }
}

pub struct IanaAllocated(pub u64);

impl From<u64> for IanaAllocated {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

pub fn is_iana_reversed<T>(s: T) -> bool
where
    T: Into<IanaAllocated>,
{
    let s = s.into();
    s.0 == 0 || s.0 == 65535
}

fn main() {
    println!("Hello, world!");
    let puppy_dog = PuppyDog {
        name: String::from("lion"),
        age: 2,
    };
    let dog = Dog::try_from(puppy_dog).unwrap();
    println!("{:#?}", dog);

    if is_iana_reversed(2) {
        println!("is_iana_reversed");
    } else {
        println!("is_not_iana_reversed");
    }
}
