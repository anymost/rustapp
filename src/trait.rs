struct Singer {
    name: String
}

trait Artist {
    fn play(&self);
    fn live(&self) {
        println!("I have to live");
    }
}

trait Worker {
    fn work(&self);
}

impl Artist for Singer {
    fn play(&self) {
        println!("{} play", self.name)
    }
}

impl Worker for Singer {
    fn work(&self) {
        println!("I am working");
    }
}

// impl trait
fn play(player: impl Artist) {
    player.play();
    player.live();
}

// trait bound
fn play1<T: Artist>(player: T) {
    player.play();
    player.live();
}

// + trait
fn play2<T: Artist+Worker>(player: T) {
    player.play();
    player.live();
    player.work();
}

// where trait
fn play3<T>(player: T)
 where T: Artist+Worker {
     player.play();
     player.live();
     player.work();
}


fn make_player() -> impl Artist {
    Singer{
        name: String::from("jack")
    }
}

fn largest<T: PartialOrd+Copy>(array: &[T]) -> T {
    let mut val = array[0];
    for &item in array.iter() {
        if item > val {
            val = item;
        } 
    }
    val
}

fn main() {
    let artist = Singer{
        name: String::from("jack")
    };
    let artist2 = make_player();
    let artist3 = Singer{
        name: String::from("jack")
    };
    let artist4 = Singer{
        name: String::from("jack")
    };
    play(artist);
    play1(artist2);
    play2(artist3);
    play3(artist4);

    let array = [1, 2, 3, 4, 5, 6];
    let val = largest(&array);
    println!("{}", val);
}