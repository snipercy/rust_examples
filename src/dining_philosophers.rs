use std::thread;

struct Philosopher{
    name: String,
}

impl Philosopher{
    fn new(name: &str) -> Philosopher{
        Philosopher{
            name: name.to_string(),
        }
    }

    fn eat(&self){
        println!("{} is eating", self.name);

        thread::sleep_ms(10);

        println!("{} is done eating", self.name);
    }
}

fn main(){
    let v = vec![
        Philosopher::new("p1"),
        Philosopher::new("p2"),
        Philosopher::new("p3"),
        Philosopher::new("p4"),
        Philosopher::new("p5"),
    ];

    let handles: Vec<_> = v.into_iter().map(|p| {
        thread::spawn(move || {
            p.eat();
        })
    }).collect();

    for h in handles{
        h.join().unwrap();
    }
}
