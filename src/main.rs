use std::fmt::{Display, Formatter};
use std::iter;
use std::io::stdin;
use std::str::FromStr;
use std::ops::AddAssign;

fn main() {
    println!("Welcome!").unwrap();
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

    let actions: Vec<Action> = vec![
        Action {
            name: String::from("Eat"),
            need: Need::Hunger(0.0),
            influence: 0.3,
            progress_influence: 0.0,
        },
        Action {
            name: String::from("Sleep"),
            need: Need::Sleep(0.0),
            influence: 0.3,
            progress_influence: 0.0,
        },
        Action {
            name: String::from("Coffee"),
            need: Need::Sleep(0.0),
            influence: 0.25,
            progress_influence: 0.05,
        },
        Action {
            name: String::from("Learn"),
            need: Need::Sleep(0.0),
            influence: -0.05,
            progress_influence: 0.15,
        },
        Action {
            name: String::from("Alcohol"),
            need: Need::Fun(0.0),
            influence: 0.3,
            progress_influence: -0.1,
        },
        Action {
            name: String::from("Procrastinate"),
            need: Need::Fun(0.0),
            influence: 0.05,
            progress_influence: -0.1,
        },
        Action {
            name: String::from("Play games"),
            need: Need::Fun(0.0),
            influence: 0.2,
            progress_influence: -0.05,
        },
    ];

    let mut buf = String::with_capacity(1);

    for _ in 0u16..60*3u16 {
        println!("{}", state);
        println!("What do you want to do?");
        for (i, action) in actions.iter().enumerate() {
            println!("{}. {}", i, action).unwrap();
        }
        stdin().read_line(&mut buf).expect("WRONG");
        let input = usize::from_str(&buf.trim()).expect("WRONG");
        let action = actions.get(input).expect("WRONG");
        action.apply(&mut state);
        for need in vec![&mut state.hunger, &mut state.sleep, &mut state.fun] {
            need.add_assign(-0.1);
            match need {
                Need::Hunger(f) if *f <= 0.0 => panic!("GAME OVER"),
                Need::Sleep(f) if *f <= 0.0 => panic!("GAME OVER"),
                Need::Fun(f) if *f <= 0.0 => panic!("GAME OVER"),
                _ => {}
            }
        }
        buf.clear();
    }
    if state.progress.progress < state.progress.limits[0] {
        println!("FAILED").unwrap();
    } else {
        println!("Success").unwrap();
    }
}

struct State {
    hunger: Need,
    sleep: Need,
    fun: Need,
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
        for need in vec![&self.hunger, &self.fun, &self.sleep] {
            writeln!(f, "{}", need);
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
        let progress: String = iter::repeat("=").take(fill).chain(iter::repeat(" ").take(unfill)).collect();
        write!(f, "{}: [{}]", name, progress)
    }
}

impl Display for Progress {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let width = f.width().unwrap_or(16);
        let fill = ((width - 10) as f32 * self.progress) as usize;
        let unfill = (width - 10) - fill;

        let progress: String = iter::repeat("=").take(fill).chain(iter::repeat(" ").take(unfill)).collect();
        write!(f, "Progress: [{}]", progress)
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {} {}{} - Progress {}{}", self.name, self.need, match self.influence.signum() {
            1.0 => "+",
            -1.0 => "-",
            _ => " "
        }, self.influence.abs(), match self.progress_influence.signum() {
            1.0 => "+",
            -1.0 => "-",
            _ => " "
        }, self.progress_influence.abs())
    }
}

impl AddAssign<f32> for Need {
    fn add_assign(&mut self, rhs: f32) {
        match self {
            Need::Hunger(v) => v.add_assign(rhs),
            Need::Sleep(v) => v.add_assign(rhs),
            Need::Fun(v) => v.add_assign(rhs)
        }
    }
}

impl AddAssign<f32> for Progress {
    fn add_assign(&mut self, rhs: f32) {
        self.progress += rhs
    }
}

impl Action {
    fn apply(&self, state: &mut State) {
        match self.need {
            Need::Hunger(_) => state.hunger += self.influence + 0.1,
            Need::Sleep(_) => state.sleep += self.influence + 0.1,
            Need::Fun(_) => state.fun += self.influence + 0.1
        };
        state.progress += self.progress_influence;
    }
}