use num_traits::int::PrimInt;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Binary,
    hash::Hash,
};

pub mod expr_dis;
pub mod judge;

pub use expr_dis::display_expr;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum StepMethod<T> {
    Not(T),
    And(T, T),
    Or(T, T),
    Exist,
    Magic,
}
#[derive(Clone, Copy, Debug)]
pub struct Step<S, T>(S, StepMethod<T>);
impl<S: PrimInt, T> PartialEq for Step<S, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<S: PrimInt, T> Eq for Step<S, T> {}
impl<S: PrimInt, T> PartialOrd for Step<S, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<S: PrimInt, T> Ord for Step<S, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

pub enum Expr<T> {
    Not(Box<Expr<T>>),
    And(Box<Expr<T>>, Box<Expr<T>>),
    Or(Box<Expr<T>>, Box<Expr<T>>),
    Exist(T),
    Magic,
}

pub struct Maker<S, T> {
    pub factors: HashSet<T>,
    makings: HashSet<T>,
    pub step_map: HashMap<T, Step<S, T>>,
}

impl<S: PrimInt, T: PrimInt + Hash + Binary> Maker<S, T> {
    pub fn new(factors: HashSet<T>) -> Maker<S, T> {
        let step_map = HashMap::from_iter(
            factors
                .iter()
                .map(|&factor| (factor, Step(S::zero(), StepMethod::Exist))),
        );
        Maker {
            factors,
            makings: HashSet::new(),
            step_map,
        }
    }
    pub fn make(&mut self, target: T) -> Step<S, T> {
        if self.makings.contains(&target) {
            return Step(S::max_value(), StepMethod::Magic);
        }
        if let Some(&step) = self.step_map.get(&target) {
            // println!("exist {:0>8}", format!("{:b}", target));
            return step;
        }
        // println!("making {:0>8}", format!("{:b}", target));
        self.makings.insert(target);
        let step = self
            .test_and(target)
            .min(self.test_not(target))
            .min(self.test_or(target));
        self.makings.remove(&target);
        if step.1 != StepMethod::Magic {
            self.step_map.insert(target, step);
            println!("success {:0>8}", format!("{:b}", target));
        } else {
            // println!("failed {:0>8}", format!("{:b}", target));
        }
        step
    }
    pub fn test_and(&mut self, target: T) -> Step<S, T> {
        // println!("andin {:0>8}", format!("{:b}", target));
        let mut min_step = Step(S::max_value(), StepMethod::Magic);
        let mut check = target;
        let mut counter = T::one();
        let mut pos_ori: Vec<T> = vec![];
        while counter != T::zero() {
            if check >> 1 << 1 == check {
                pos_ori.push(counter);
            }
            counter = counter << 1;
            check = check >> 1;
        }
        let mut pos = vec![];
        for n in pos_ori {
            let step = self.make(target | n);
            if step.1 != StepMethod::Magic {
                pos.push(n | target);
            }
        }
        for i in 0..pos.len() {
            for j in 0..i {
                let a = target | pos[i];
                let b = target | pos[j];
                if a & b == target {
                    let a_step = self.make(a);
                    let b_step = self.make(b);
                    min_step = min_step.min(Step(a_step.0 + b_step.0, StepMethod::And(a, b)));
                }
            }
        }
        // println!("andout {:0>8}", format!("{:b}", target));
        min_step
    }
    pub fn test_or(&mut self, target: T) -> Step<S, T> {
        // println!("orin {:0>8}", format!("{:b}", target));
        let mut min_step = Step(S::max_value(), StepMethod::Magic);
        let mut check = target;
        let mut counter = T::one();
        let mut pos_ori: Vec<T> = vec![];
        while counter != T::zero() {
            if check >> 1 << 1 != check {
                pos_ori.push(counter);
            }
            counter = counter << 1;
            check = check >> 1;
        }
        let mut pos = vec![];
        for n in pos_ori {
            let step = self.make(target - n);
            if step.1 != StepMethod::Magic {
                pos.push(target - n);
            }
        }
        for i in 0..pos.len() {
            for j in 0..i {
                let a = target - pos[i];
                let b = target - pos[j];
                if a | b == target {
                    let a_step = self.make(a);
                    let b_step = self.make(b);
                    min_step = min_step.min(Step(a_step.0 + b_step.0, StepMethod::Or(a, b)));
                }
            }
        }
        // println!("orout {:0>8}", format!("{:b}", target));
        min_step
    }
    fn test_not(&mut self, target: T) -> Step<S, T> {
        let step = self.make(!target);
        if step.1 == StepMethod::Magic {
            return step;
        }
        Step(step.0 + S::one(), StepMethod::Not(!target))
    }
    pub fn get_expr(&self, target: T) -> Expr<T> {
        match *self
            .step_map
            .get(&target)
            .unwrap_or(&Step(S::max_value(), StepMethod::Magic))
        {
            Step(_, StepMethod::And(a, b)) => {
                Expr::And(Box::new(self.get_expr(a)), Box::new(self.get_expr(b)))
            }
            Step(_, StepMethod::Or(a, b)) => {
                Expr::Or(Box::new(self.get_expr(a)), Box::new(self.get_expr(b)))
            }
            Step(_, StepMethod::Not(n)) => Expr::Not(Box::new(self.get_expr(n))),
            Step(_, StepMethod::Exist) => Expr::Exist(target),
            Step(_, StepMethod::Magic) => Expr::Magic,
        }
    }
    pub fn check(&self, target: T) -> bool {
        judge::is_makable(target, &self.factors)
    }
    pub fn check_detail(&self, target: T) -> HashSet<T> {
        judge::is_makable_detail(target, &self.factors)
    }
}
