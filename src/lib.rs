use num_traits::int::PrimInt;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Binary,
    hash::Hash,
};

pub mod expr_dis;
pub mod judge;

pub use expr_dis::display_expr;

pub enum Step<T> {
    Not(T),
    And(T, T),
    Or(T, T),
    Exist,
    Magic,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Route<T> {
    Not,
    And(T),
    Or(T),
}

pub enum Expr<T> {
    Not(Box<Expr<T>>),
    And(Box<Expr<T>>, Box<Expr<T>>),
    Or(Box<Expr<T>>, Box<Expr<T>>),
    Exist(T),
    Magic,
}

pub struct Maker<T> {
    pub factors: HashSet<T>,
    pub target: T,
    targets: VecDeque<T>,
    pub step_map: HashMap<T, Step<T>>,
    pub routes: HashMap<T, HashSet<Route<T>>>,
}

impl<T: PrimInt + Hash + Binary> Maker<T> {
    pub fn new(factors: HashSet<T>, target: T) -> Maker<T> {
        let step_map: HashMap<T, Step<T>> =
            HashMap::from_iter(factors.iter().map(|&factor| (factor, Step::Exist)));
        let routes: HashMap<T, HashSet<Route<T>>> =
            HashMap::from_iter(factors.iter().map(|&factor| (factor, HashSet::new())));
        Maker {
            factors,
            target,
            targets: VecDeque::from([target]),
            step_map,
            routes,
        }
    }
    fn deal_route(&mut self, route: Route<T>, target: T, fined: bool) -> bool {
        if fined {
            match route {
                Route::Not => {
                    if !self.step_map.contains_key(&!target) {
                        self.step_map.insert(!target, Step::Not(target));
                        if self.fine(!target) {
                            return true;
                        }
                    }
                }
                Route::And(n) => {
                    if self.step_map.contains_key(&n) && !self.step_map.contains_key(&(n & target))
                    {
                        self.step_map.insert(n & target, Step::And(n, target));
                        if self.fine(n & target) {
                            return true;
                        }
                    }
                }
                Route::Or(n) => {
                    if self.step_map.contains_key(&n) && !self.step_map.contains_key(&(n | target))
                    {
                        self.step_map.insert(n | target, Step::Or(n, target));
                        if self.fine(n | target) {
                            return true;
                        }
                    }
                }
            }
        } else {
            self.routes
                .entry(target)
                .or_insert(HashSet::new())
                .insert(route);
        }
        false
    }
    fn fine(&mut self, target: T) -> bool {
        // println!("fine {:0>8}", format!("{:b}", target));
        let routes = self.routes.remove(&target);
        if target == self.target {
            return true;
        }
        for route in routes.unwrap() {
            if self.deal_route(route, target, true) {
                return true;
            }
        }
        false
    }
    pub fn make(&mut self) {
        let mut e: usize = 400;
        while let Some(target) = self.targets.pop_front() {
            if self.factors.remove(&target) {
                // println!("success {:0>8}", format!("{:b}", target));
                if self.fine(target) {
                    return;
                }
            } else if !self.step_map.contains_key(&target) {
                // println!("making {:0>8}", format!("{:b}", target));
                if self.test_and(target) || self.test_or(target) || self.test_not(target) {
                    return;
                }
            }
            e -= 1;
            if e == 0 {
                break;
            }
        }
    }
    pub fn test_and(&mut self, target: T) -> bool {
        // println!("andin {:0>8}", format!("{:b}", target));
        let mut check = target;
        let mut counter = T::one();
        let mut pos_ori: Vec<T> = vec![];
        let mut size = T::zero();
        while counter != T::zero() {
            if check >> 1 << 1 == check {
                pos_ori.push(counter);
                size = (size << 1) + T::one();
            }
            counter = counter << 1;
            check = check >> 1;
        }
        let mut pos = vec![];
        loop {
            counter = T::one();
            check = T::zero();
            for no in 0..pos_ori.len() {
                if size | counter == size {
                    check = check | pos_ori[no];
                }
                counter = counter << 1;
            }
            pos.push(check);
            if size == T::zero() {
                break;
            }
            size = size - T::one();
        }
        for i in 0..pos.len() {
            if !self.step_map.contains_key(&(target | pos[i])) {
                self.targets.push_back(target | pos[i]);
            }
            for j in 0..i {
                let a = target | pos[i];
                let b = target | pos[j];
                if a & b == target {
                    if self.deal_route(Route::And(b), a, self.step_map.contains_key(&a)) {
                        return true;
                    }
                    if self.deal_route(Route::And(a), b, self.step_map.contains_key(&b)) {
                        return true;
                    }
                }
            }
        }
        false
        // println!("andout {:0>8}", format!("{:b}", target));
    }
    pub fn test_or(&mut self, target: T) -> bool {
        // println!("orin {:0>8}", format!("{:b}", target));
        let mut check = target;
        let mut counter = T::one();
        let mut pos_ori: Vec<T> = vec![];
        let mut size = T::zero();
        while counter != T::zero() {
            if check >> 1 << 1 != check {
                pos_ori.push(counter);
                size = (size << 1) + T::one();
            }
            counter = counter << 1;
            check = check >> 1;
        }
        let mut pos = vec![];
        loop {
            counter = T::one();
            check = T::zero();
            for no in 0..pos_ori.len() {
                if size | counter == size {
                    check = check | pos_ori[no];
                }
                counter = counter << 1;
            }
            pos.push(check);
            if size == T::zero() {
                break;
            }
            size = size - T::one();
        }
        for i in 0..pos.len() {
            if !self.step_map.contains_key(&(target - pos[i])) {
                self.targets.push_back(target - pos[i]);
            }
            for j in 0..i {
                let a = target - pos[i];
                let b = target - pos[j];
                if a | b == target {
                    if self.deal_route(Route::Or(b), a, self.step_map.contains_key(&a)) {
                        return true;
                    }
                    if self.deal_route(Route::Or(a), b, self.step_map.contains_key(&b)) {
                        return true;
                    }
                }
            }
        }
        false
        // println!("orout {:0>8}", format!("{:b}", target));
    }
    fn test_not(&mut self, target: T) -> bool {
        let fined = self.step_map.contains_key(&target);
        if !fined {
            self.targets.push_back(!target);
        }
        self.deal_route(Route::Not, target, fined)
    }
    pub fn get_expr(&self, target: T) -> Expr<T> {
        match *self.step_map.get(&target).unwrap_or(&Step::Magic) {
            Step::And(a, b) => Expr::And(Box::new(self.get_expr(a)), Box::new(self.get_expr(b))),
            Step::Or(a, b) => Expr::Or(Box::new(self.get_expr(a)), Box::new(self.get_expr(b))),
            Step::Not(n) => Expr::Not(Box::new(self.get_expr(n))),
            Step::Exist => Expr::Exist(target),
            Step::Magic => Expr::Magic,
        }
    }
    pub fn check(&self, target: T) -> bool {
        judge::is_makable(target, &self.factors)
    }
    pub fn check_detail(&self, target: T) -> HashSet<T> {
        judge::is_makable_detail(target, &self.factors)
    }
}
