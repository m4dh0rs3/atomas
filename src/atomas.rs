pub(crate) struct Atomas {
    arena: Arena,
    action: Option<Action>,
    mode: Mode,
    level: Element,
    // buff: Element,
}

#[derive(Clone, PartialEq, Default)]
pub(crate) enum Mode {
    #[default]
    Classic,
    TimeAttack,
    Zen,
    Geneva,
}

enum Action {
    Particle(Particle),
    Electron(Option<Element>),
    Luxon,
}

#[derive(Debug)]
pub(crate) struct Particle {
    reactive: Option<Reactor>,
    element: Element,
}

#[derive(PartialEq, Debug)]
enum Reactor {
    Proton,
    AntiProton,
    AntiMatter,
}

type Element = u8;

/* #[derive(Clone, PartialEq, Debug)]
#[rustfmt::skip]
enum Element {
    H = 1,
    He = 2,
    Li = 3,
    Be = 4,

    B = 5,
    C = 6,
    N = 7,
    O = 8,
    F = 9,
    Ne = 10,

    Na = 11,
    Mg = 12,

    Al = 13,
    Si = 14,
    P = 15,
    S = 16,
    Cl = 17,
    Ar = 18,

    K = 19,
    Ca = 20,

    Sc = 21,
    Ti = 22,
    V = 23,
    Cr = 24,
    Mn = 25,
    Fe = 26,
    Co = 27,
    Ni = 28,
    Cu = 29,
    Zn = 30,

    Ga = 31,
    Ge = 32,
    As = 33,
    Se = 34,
    Br = 35,
    Kr = 36,

    Rb = 37,
    Sr = 38,

    Y = 39,
    Zr = 40,
    Nb = 41,
    Mo = 42,
    Tc = 43,
    Ru = 44,
    Rh = 45,
    Pd = 46,
    Ag = 47,
    Cd = 48,

    In = 49,
    Sn = 50,
    Sb = 51,
    Te = 52,
    I = 53,
    Xe = 54,

    Cs = 55,
    Ba = 56,

    Lu = 71,
    Hf = 72,
    Ta = 73,
    W = 74,
    Re = 75,
    Os = 76,
    Ir = 77,
    Pt = 78,
    Au = 79,
    Hg = 80,

    Tl = 81,
    Pb = 82,
    Bi = 83,
    Po = 84,
    At = 85,
    Rn = 86,

    Fr = 87,
    Ra = 88,

    Lr = 103,
    Rf = 104,
    Db = 105,
    Sg = 106,
    Bh = 107,
    Hs = 108,
    Mt = 109,
    Ds = 110,
    Rg = 111,
    Cn = 112,

    Nh = 113,
    Fl = 114,
    Mc = 115,
    Lv = 116,
    Ts = 117,
    Og = 118,

    La = 57,
    Ce = 58,
    Pr = 59,
    Nd = 60,
    Pm = 61,
    Sm = 62,
    Eu = 63,
    Gd = 64,
    Tb = 65,
    Dy = 66,
    Ho = 67,
    Er = 68,
    Tm = 69,
    Yb = 70,

    Ac = 89,
    Th = 90,
    Pa = 91,
    U = 92,
    Np = 93,
    Pu = 94,
    Am = 95,
    Cm = 96,
    Bk = 97,
    Cf = 98,
    Es = 99,
    Fm = 100,
    Md = 101,
    No = 102,
} */

#[derive(PartialEq, Debug)]
pub(crate) struct Arena(Vec<Particle>);

impl Atomas {
    pub(crate) fn new(mode: Mode) -> Self {
        Self {
            arena: Arena::new(),
            action: None,
            mode,
            level: 1,
        }
    }

    pub(crate) fn get_arena(&self) -> &Arena {
        &self.arena
    }

    pub(crate) fn act(&mut self, index: usize) {
        if let Some(action) = self.action.take() {
            match action {
                Action::Particle(particle) => self.arena.insert(index, particle),
                Action::Electron(_) => todo!(),
                Action::Luxon => todo!(),
            }
        }

        self.generate()
    }

    pub(crate) fn convert(&mut self) {
        if let Some(action @ Action::Electron(Some(_))) = &mut self.action {
            *action = Action::Particle(Particle {
                element: 1,
                reactive: Some(Reactor::Proton),
            })
        }
    }

    fn generate(&mut self) {
        if let None = self.action {}
    }
}

impl Arena {
    fn new() -> Self {
        // TODO: Generate some default contents?
        Self(vec![
            Particle {
                reactive: None,
                element: 3,
            },
            Particle {
                reactive: Some(Reactor::Proton),
                element: 1,
            },
            Particle {
                reactive: None,
                element: 3,
            },
        ])
    }

    pub(crate) fn get_particles(&self) -> &Vec<Particle> {
        &self.0
    }

    fn wrap_index(&self, index: isize) -> usize {
        index.rem_euclid(self.0.len() as isize) as usize
    }

    fn insert(&mut self, index: usize, particle: Particle) {
        self.0.insert(index, particle);
        self.react(index as isize);
        self.react(index as isize + 1);
        self.react(index as isize - 1);
    }

    fn remove(&mut self, index: isize) -> Particle {
        self.0.remove(self.wrap_index(index))
    }

    fn react(&mut self, index: isize) {
        if let Some(reactor) = &self[index].reactive {
            match reactor {
                Reactor::Proton => {
                    if self[index - 1] == self[index + 1] {
                        let (index, left, _) = self.remove_adjacent(index);
                        self[index].fuse(left);
                    } else if self[index].element != 1 {
                        self[index].reactive = None;
                    }
                }
                Reactor::AntiProton => todo!(),
                Reactor::AntiMatter => todo!(),
            }
        }
    }

    fn remove_adjacent(&mut self, mut index: isize) -> (isize, Particle, Particle) {
        let shift = self.wrap_index(index) > 0;
        let left = self.remove(index - 1);
        if shift {
            index -= 1;
        }
        let shift = self.wrap_index(index) > 0;
        let right = self.remove(index + 1);
        if shift {
            index -= 1;
        }

        (index, left, right)
    }
}

impl Particle {
    fn fuse(&mut self, other: Self) {
        self.element = self.element.max(other.element)
            + if self.element > 1 && other.element > self.element {
                other.element - self.element
            } else {
                1
            }
    }

    pub(crate) fn get_element(&self) -> &Element {
        &self.element
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.reactive == None && other.reactive == None && self.element == other.element
    }
}

use std::ops;

impl ops::Index<usize> for Arena {
    type Output = Particle;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::Index<isize> for Arena {
    type Output = Particle;

    fn index(&self, index: isize) -> &Self::Output {
        &self[self.wrap_index(index)]
    }
}

impl ops::IndexMut<usize> for Arena {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl ops::IndexMut<isize> for Arena {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        let index = self.wrap_index(index);
        &mut self[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_index() {
        let arena = Arena(vec![
            Particle {
                reactive: None,
                element: 3,
            },
            Particle {
                reactive: Some(Reactor::Proton),
                element: 1,
            },
            Particle {
                reactive: None,
                element: 3,
            },
        ]);

        assert_eq!(arena.wrap_index(3), 0);
        assert_eq!(arena.wrap_index(0), 0);
        assert_eq!(arena.wrap_index(1), 1);
        assert_eq!(arena.wrap_index(2), 2);
        assert_eq!(arena.wrap_index(-1), 2);
    }

    #[test]
    fn fuse() {
        let mut arena = Arena(vec![
            Particle {
                reactive: None,
                element: 3,
            },
            Particle {
                reactive: Some(Reactor::Proton),
                element: 1,
            },
            Particle {
                reactive: None,
                element: 3,
            },
        ]);

        println!("{:?}", arena);
        arena.react(1);
        println!("{:?}", arena);

        let mut arena = Arena(vec![
            Particle {
                reactive: None,
                element: 3,
            },
            Particle {
                reactive: None,
                element: 3,
            },
            Particle {
                reactive: Some(Reactor::Proton),
                element: 1,
            },
        ]);

        println!("\n{:?}", arena);
        arena.react(2);
        println!("{:?}", arena);

        let mut arena = Arena(vec![
            Particle {
                reactive: Some(Reactor::Proton),
                element: 1,
            },
            Particle {
                reactive: None,
                element: 3,
            },
            Particle {
                reactive: None,
                element: 3,
            },
        ]);

        println!("\n{:?}", arena);
        arena.react(0);
        println!("{:?}", arena);

        let mut arena = Arena(vec![
            Particle {
                reactive: Some(Reactor::Proton),
                element: 1,
            },
            Particle {
                reactive: None,
                element: 3,
            },
            Particle {
                reactive: None,
                element: 4,
            },
            Particle {
                reactive: None,
                element: 3,
            },
        ]);

        println!("\n{:?}", arena);
        arena.react(0);
        println!("{:?}", arena);
    }
}
