use quadtree::*;

pub fn bottom(tree: &mut QuadTree) -> Option<&mut Box<QuadTree>> {
    unsafe {
        match tree.pos {
            Pos::TOP => return None,
            Pos::TL => return Some((*tree.parent).bl.as_mut().unwrap()),
            Pos::TR => return Some((*tree.parent).br.as_mut().unwrap()),
            Pos::BL => {
                match bottom(&mut *tree.parent) {
                    None => return None,
                    Some(x) => {
                        if x.is_leaf {
                            return Some(x);
                        }
                        return Some(x.tl.as_mut().unwrap());
                    }
                }
            },
            Pos::BR => {
                match bottom(&mut *tree.parent) {
                    None => return None,
                    Some(x) => {
                        if x.is_leaf {
                            return Some(x);
                        }
                        return Some(x.tr.as_mut().unwrap());
                    }
                }
            },
        }
    }
}

pub fn top(tree: &mut QuadTree) -> Option<&mut Box<QuadTree>> {
    unsafe {
        match tree.pos {
            Pos::TOP => return None,
            Pos::BL => return Some((*tree.parent).tl.as_mut().unwrap()),
            Pos::BR => return Some((*tree.parent).tr.as_mut().unwrap()),
            Pos::TL => {
                match top(&mut *tree.parent) {
                    None => return None,
                    Some(x) => {
                        if x.is_leaf {
                            return Some(x);
                        }
                        return Some(x.bl.as_mut().unwrap());
                    }
                }
            },
            Pos::TR => {
                match top(&mut *tree.parent) {
                    None => return None,
                    Some(x) => {
                        if x.is_leaf {
                            return Some(x);
                        }
                        return Some(x.br.as_mut().unwrap());
                    }
                }
            },
        }
    }
}

pub fn right(tree: &mut QuadTree) -> Option<&mut Box<QuadTree>> {
    unsafe {
        match tree.pos {
            Pos::TOP => return None,
            Pos::TL => return Some((*tree.parent).tr.as_mut().unwrap()),
            Pos::BL => return Some((*tree.parent).br.as_mut().unwrap()),
            Pos::TR => {
                match left(&mut *tree.parent) {
                    None => return None,
                    Some(x) => {
                        if x.is_leaf {
                            return Some(x);
                        }
                        return Some(x.tl.as_mut().unwrap());
                    }
                }
            },
            Pos::BR => {
                match left(&mut *tree.parent) {
                    None => return None,
                    Some(x) => {
                        if x.is_leaf {
                            return Some(x);
                        }
                        return Some(x.bl.as_mut().unwrap());
                    }
                }
            },
        }
    }
}

pub fn left(tree: &mut QuadTree) -> Option<&mut Box<QuadTree>> {
    unsafe {
        match tree.pos {
            Pos::TOP => return None,
            Pos::TR => return Some((*tree.parent).tl.as_mut().unwrap()),
            Pos::BR => return Some((*tree.parent).bl.as_mut().unwrap()),
            Pos::TL => {
                match left(&mut *tree.parent) {
                    None => return None,
                    Some(x) => {
                        if x.is_leaf {
                            return Some(x);
                        }
                        return Some(x.tr.as_mut().unwrap());
                    }
                }
            },
            Pos::BL => {
                match left(&mut *tree.parent) {
                    None => return None,
                    Some(x) => {
                        if x.is_leaf {
                            return Some(x);
                        }
                        return Some(x.br.as_mut().unwrap());
                    }
                }
            },
        }
    }
}
