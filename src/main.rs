use std::fmt::{Display, Formatter};
use std::iter;
use std::io::stdin;
use std::str::FromStr;

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

    let actions: Vec<Action> = vec![];

    let mut buf = String::with_capacity(2);

    for t in 0u16..60*3u16 {
        writeln!(t, "{}", state);
        writeln!(t, "What do you want to do?");
        for (i, action) in actions.iter().enumerate() {
            writeln!(t, "{}. {}", i, action)
        }
        stdin().read_line(&mut buf).expect("WRONG");
        let input = usize::from_str(&buf).expect("WRONG");
        let action = actions.get(input).expect("WRONG");

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

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for need in (self.hunger, self.fun, self.sleep) {
            writeln!(f, "{}", need)
        }
        writeln!(f, "{}", self.progress);
        Ok(())
    }
}

impl Display for Need {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let (name, v) = match self {
            Need::Hunger(v) => ("Hunger", v),
            Need::Sleep(v) =>  (" Sleep", v),
            Need::Fun(v) =>    ("   Fun", v)
        };
        let width = f.width().unwrap_or(16);
        let fill = ((width - 10) as f32 * v) as usize;
        let unfill = (width - 10) - fill;
        write!(f, "{}: [{}]", name, iter::repeat('🟩').take(fill).chain(iter::repeat('⬜').take(unfill)).collect() as String)
    }
}

impl Display for Progress {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let width = f.width().unwrap_or(16);
        let fill = ((width - 10) as f32 * v) as usize;
        let unfill = (width - 10) - fill;

        write!(f, "Progress: [{}]", iter::repeat('🟩').take(fill).chain(iter::repeat('⬜').take(unfill)).collect() as String)
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {} {}{} - Progress {}{}", self.name, self.need, self.influence.signum(), self.influence.abs(), self.progress_influence.signum(), self.progress_influence.abs())
    }
}

impl Action {

}