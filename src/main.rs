fn main() {
    let mut t = term::stdout().unwrap();
    writeln!(t, "Welcome!").unwrap();
    let mut progress = Progress {
        limits: vec![0.5, 0.8, 1.0],
        progress: 0.0
    };

    let mut state = State {
        hunger: Need::Hunger(0.5),
        sleep: Need::Sleep(0.5),
        fun: Need::Fun(0.5),
        progress
    };



    for t in 0u16..60*3u16 {
        println!("What do you want to do?")
    }
}

struct State {
    hunger: Need::Hunger,
    sleep: Need::Sleep,
    fun: Need::Fun,
    progress: Progress
}

struct Action {
    need: Need,
    name: String,
    influence: f32,
    progress_influence: f32
}

enum Need {
    Hunger(f32),
    Sleep(f32),
    Fun(f32)
}

struct Progress {
    limits: Vec<f32>,
    progress: f32
}
