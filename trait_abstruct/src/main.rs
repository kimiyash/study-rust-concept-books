trait SayHello {
    fn say_hello(&self);
}

trait SayThankyou {
    fn say_thankyou(&self);
}

trait Run {
    fn run(&self);
}

struct EnglishPerson;
struct SpanisPerson;

impl SayHello for EnglishPerson {
    fn say_hello(&self) {
        println!("Hello");
    }
}

impl SayThankyou for EnglishPerson {
    fn say_thankyou(&self) {
        println!("Thankyou");
    }
}

impl Run for EnglishPerson {
    fn run(&self) {
        println!("Run");
    }
}

impl SayHello for SpanisPerson {
    fn say_hello(&self) {
        println!("Hola")
    }
}

impl SayThankyou for SpanisPerson {
    fn say_thankyou(&self) {
        println!("Gracias");
    }    
}

impl Run for SpanisPerson {
    fn run(&self) {
        println!("Correr");
    }
    
}

fn say_hello_general<T: SayHello>(speaker: &T) {
    speaker.say_hello();
}

fn say_thankyou_general<T: SayThankyou>(speaker: &T) {
    speaker.say_thankyou();
}

fn say_thankyou_and_run<T: SayThankyou + Run>(person: &T)
{
    person.say_thankyou();
    person.run();
}

fn main() {
    let en = EnglishPerson;
    let sp = SpanisPerson;

    say_hello_general(&en);
    say_hello_general(&sp);

    say_thankyou_general(&en);
    say_thankyou_general(&sp);

    say_thankyou_and_run(&en);
    say_thankyou_and_run(&sp);
}
