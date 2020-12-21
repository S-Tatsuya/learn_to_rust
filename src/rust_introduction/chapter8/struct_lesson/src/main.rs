struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

fn print_person(pa : &Person) {
    println!("{}: {} ({}) in {}",
        pa.id, pa.name, pa.age, pa.addr);
}

fn add_age(pa: &mut Person) {
    pa.age += 1;
}

fn new_person(id: i32, name: &str) -> Person {
    let pa = Person {
        id: id,
        name: name.to_string(),
        age: -1,
        addr: String::from("Unknown"),
    };
    pa
}

fn main() {
    let mut pa = Person {
        id: 100,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };

    println!("{}: {} ({}) in {}", pa.id, pa.name, pa.age, pa.addr);
    pa.age += 1;
    println!("{}: {} ({}) in {}", pa.id, pa.name, pa.age, pa.addr);
    pa.addr = String::from("Osaka");
    println!("{}: {} ({}) in {}", pa.id, pa.name, pa.age, pa.addr);

    // function argv in struct
    let mut pa = Person{
        id: 100,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };

    print_person(&pa);

    add_age(&mut pa);
    print_person(&pa);


    // function return in struct
    let pa2 = new_person(200, "kato");
    println!("{}: {} ({}) in {}",
        pa2.id, pa2.name, pa2.age, pa2.addr);

    // vector in struct
    let pa = new_person(100, "masdua");
    let pa2 = new_person(200, "kato");

    let mut people = vec![pa, pa2];
    people.push(new_person(300, "yamada"));
    people.push(new_person(400, "sato"));

    for p in &people {
        print_person(p);
    }

    // no filed struct
    struct Color(f32, f32, f32);
    let yellow = Color(1.0, 1.0, 1.0);
    println!("R:{:.2}, G:{:.2}, B:{:.2}", 
        yellow.0, yellow.1, yellow.2);

    // struct size
    println!("A size_of is {}", std::mem::size_of::<A>());
    println!("B size_of is {}", std::mem::size_of::<B>());
    println!("C size_of is {}", std::mem::size_of::<C>());
    println!("D size_of is {}", std::mem::size_of::<D>());
    println!("E size_of is {}", std::mem::size_of::<E>());
    println!("F size_of is {}", std::mem::size_of::<F>());
}

struct A {
    id: i32,
}

struct B {
    id1: i32,
    id2: i32,
    id3: i32,
}

struct C {
    id: i32,
    name: String,
}

struct D {
    id: i32,
    name: &'static str,
}

struct E {
    id: i32,
    v: Vec<u8>,
}

struct F {
    id:i32,
    v: [u8; 100],
}