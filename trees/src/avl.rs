use std::{fmt::Debug, mem, cmp::Ordering};

pub struct Avl<T: Ord> {
    tr: Option<Box<Tree<T>>>,
}

struct Tree<T: Ord> {
    val: T,
    ltr: Option<Box<Tree<T>>>,
    rtr: Option<Box<Tree<T>>>,
    depth: i32,
}

enum TreeState {
    LL,
    RR,
    LR,
    RL,
    OK,
}

impl<T: Debug + Ord> Debug for Tree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tree")
            .field("val", &self.val)
            .field("depth", &self.depth)
            .field("ltr", &self.ltr)
            .field("rtr", &self.rtr)
            .finish()
    }
}

impl<T: Ord> Avl<T> {

    // 根节点都需要特殊处理
    pub fn new() -> Self {
        Avl { tr: None }
    }

    pub fn insert(&mut self, t: T) {
        match self.tr.as_mut() {
            Some(tr) => {
                tr.insert(t);
            }
            None => {
                let tr = Tree {
                    val: t,
                    ltr: None,
                    rtr: None,
                    depth: 1,
                };
                self.tr = Some(Box::new(tr));
            }
        }
    }

    pub fn delete(&mut self, t: &T) {
        let tmp = self.tr.take();
        
        self.tr = tmp.and_then(|tr| Tree::delete(tr, t));
    }

    pub fn contains(&self, t: &T) -> bool {
        match self.tr.as_ref() {
            Some(tr) => tr.contains(t),
            None => false,
        }
    }

    #[allow(unused)]
    fn check_rule(&self) -> bool {
        match self.tr.as_ref() {
            Some(tr) => tr.check_rules(),
            None => true,
        }
    }
}

impl<T: Ord> Tree<T> {
    #[inline]
    #[allow(unused)]
    fn ldepth(&self) -> i32 {
        self.ltr.as_ref().map_or(0, |tr| tr.depth)
    }

    #[inline]
    #[allow(unused)]
    fn rdepth(&self) -> i32 {
        self.rtr.as_ref().map_or(0, |tr| tr.depth)
    }

    #[inline]
    #[allow(unused)]
    fn is_leaf(&self) -> bool {
        self.ltr.is_none() && self.rtr.is_none()
    }

    #[inline]
    #[allow(unused)]
    fn is_branch(&self) -> bool {
        !self.is_leaf()
    }

    // 左旋只需保证有右子树
    fn lrotate(&mut self) {
        assert!(self.rtr.is_some());

        let mut rtr = self.rtr.take().unwrap();
        let rl = rtr.ltr.take();
        self.rtr = rl;
        let mut oringin_root = mem::replace(self, *rtr);
        oringin_root.rebalance();
        self.ltr = Some(Box::new(oringin_root));

        // self.ltr.as_mut().unwrap().rebalance();
        self.rebalance();
    }

    // 右旋需要保证有左子树
    fn rrotate(&mut self) {
        assert!(self.ltr.is_some());

        let mut ltr = self.ltr.take().unwrap();
        let lr = ltr.rtr.take();
        self.ltr = lr;
        let mut oringin_root = mem::replace(self, *ltr);
        oringin_root.rebalance();
        self.rtr = Some(Box::new(oringin_root));

        // self.rtr.as_mut().unwrap().rebalance();
        self.rebalance();
    }

    #[allow(unused)]
    fn check_rules(&self) -> bool {
        (self.ldepth() - self.rdepth()).abs() <= 1 
        && self.depth == self.ldepth().max(self.rdepth()) + 1
        && self.ltr.as_ref().map_or(true, |tr| tr.val < self.val)
        && self.rtr.as_ref().map_or(true, |tr| tr.val > self.val)
    }

    // 输入当前树的不平衡状态，重新平衡
    fn rebalance(&mut self) {
        let state = self.update();
        match state {
            TreeState::LL => self.rrotate(),
            TreeState::RR => self.lrotate(),
            TreeState::LR => {
                self.ltr.as_mut().unwrap().lrotate();
                self.rrotate();
            }
            TreeState::RL => {
                self.rtr.as_mut().unwrap().rrotate();
                self.lrotate();
            }
            TreeState::OK => {}
        }
    }

    fn insert(&mut self, term: T) {
        match self.val.cmp(&term) {
            Ordering::Less => {
                match self.rtr.as_mut() {
                    Some(tr) => tr.insert(term),
                    None => {
                        self.rtr = Some(Box::new(Tree {
                            val: term,
                            ltr: None,
                            rtr: None,
                            depth: 1,
                        }))
                    }
                }
            },
            Ordering::Equal => {},
            Ordering::Greater => {
                match self.ltr.as_mut() {
                    Some(tr) => tr.insert(term),
                    None => {
                        self.ltr = Some(Box::new(Tree {
                            val: term,
                            ltr: None,
                            rtr: None,
                            depth: 1,
                        }))
                    }
                }
            },
        }

        self.rebalance();
    }

    // 是叶子节点 -> 直接删， 只有一个儿子 -> 删掉后儿子接上， 两个儿子 -> 找到后继节点代替后删(改变elem然后删)
    // 本来想实现一个返回mut指针，但是生命周期没法标注

    // 更新depth，返回树的状态
    fn update(&mut self) -> TreeState {
        // 假定他的子树已经更新好
        let ldepth = self.ldepth();
        let rdepth = self.rdepth();

        self.depth = ldepth.max(rdepth) + 1;

        if (ldepth - rdepth).abs() > 1 {
            if ldepth > rdepth {
                let lldepth = self.ltr.as_ref().map_or(0, |ltr| ltr.ldepth());
                let lrdepth = self.ltr.as_ref().map_or(0, |ltr| ltr.rdepth());
                if lldepth < lrdepth {
                    TreeState::LR
                } else {
                    TreeState::LL
                }
            } else {
                let rldepth = self.rtr.as_ref().map_or(0, |rtr| rtr.ldepth());
                let rrdepth = self.rtr.as_ref().map_or(0, |rtr| rtr.rdepth());
                if rldepth < rrdepth {
                    TreeState::RR
                } else {
                    TreeState::RL
                }
            }
        } else {
            TreeState::OK
        }
    }

    fn contains(&self, t: &T) -> bool {
        match self.val.cmp(t) {
            Ordering::Less => self.rtr.as_ref().map_or(false,|tr| tr.contains(t)),
            Ordering::Equal => true,
            Ordering::Greater => self.ltr.as_ref().map_or(false, |tr| tr.contains(t)),
        }
    }

    // 指向头结点的指针
    fn delete(mut root: Box<Tree<T>>, t: &T) -> Option<Box<Tree<T>>> {
        match root.val.cmp(t) {
            Ordering::Less => {
                root.rtr = root.rtr.and_then(|tr| Tree::delete(tr, &t));
                root.rebalance();
                Some(root)
            },
            Ordering::Equal => Tree::delete_root(root),
            Ordering::Greater => {
                root.ltr = root.ltr.and_then(|tr| Tree::delete(tr, &t));
                root.rebalance();
                Some(root)
            },
        }
    }

    fn delete_root(mut root: Box<Tree<T>>) -> Option<Box<Tree<T>>> {
        match root.rtr {
            Some(mut tr) => {
                if tr.ltr.is_none() {
                    // 右树的左子树没有,把根的左子树接给右树,把右树接在原来根上
                    tr.ltr = root.ltr;
                    tr.rebalance();
                    Some(tr)
                } else {
                    // 右树最左子偏的叶子即为后继节点,交换两节点的值，删去原来节点
                    root.val = Tree::left_most(&mut tr).unwrap().val;
                    root.rtr = Some(tr);
                    root.rebalance();
                    Some(root)
                }
            },
            // 无右树，接左树
            None => root.ltr,
        }
    }
    

    fn left_most(root: &mut Box<Tree<T>>) -> Option<Box<Tree<T>>> {
        match root.ltr.as_mut() {
            Some(tr) => {
                let result = Tree::left_most(tr);
                if result.is_none() {
                    let result = root.ltr.take();
                    root.rebalance();
                    result
                } else {
                    root.rebalance();
                    result
                }
            },
            None => None,
        } 
    }
}

    // 生命周期有问题


#[cfg(test)]
mod tests {
    use super::{Avl, Tree};

    #[test]
    fn rotate() {
        let mut lltr = Tree {
            val: 2,
            ltr: Some(Box::new(Tree {
                val: 1,
                ltr: {
                    Some(Box::new(Tree {
                        val: 0,
                        ltr: None,
                        rtr: None,
                        depth: 0,
                    }))
                },
                rtr: None,
                depth: 1,
            })),
            rtr: None,
            depth: 2,
        };
        println!("{:?}", lltr);
        lltr.rrotate();
        println!("右旋:");
        println!("{:?}", lltr);

        let mut rrtr = Tree {
            val: 2,
            ltr: None,
            rtr: Some(Box::new(Tree {
                val: 1,
                ltr: None,
                rtr: {
                    Some(Box::new(Tree {
                        val: 0,
                        ltr: None,
                        rtr: None,
                        depth: 0,
                    }))
                },
                depth: 1,
            })),
            depth: 2,
        };
        println!("{:?}", rrtr);
        rrtr.lrotate();
        println!("左旋:");
        println!("{:?}", rrtr);
    }

    #[test]
    fn insert() {
        let mut tr = Avl::new();

        tr.insert(1);
        tr.insert(3);
        tr.insert(5);
        tr.insert(2);
        tr.insert(4);

        assert!(tr.contains(&1));
        assert!(tr.contains(&2));
        assert!(tr.contains(&3));
        assert!(tr.contains(&4));
        assert!(tr.contains(&5));
        assert!(tr.check_rule());

        tr.delete(&2);
        assert!(tr.contains(&1));
        assert!(!tr.contains(&2));
        assert!(tr.contains(&3));
        assert!(tr.contains(&4));
        assert!(tr.contains(&5));
        assert!(tr.check_rule());

        tr.delete(&5);
        assert!(tr.contains(&1));
        assert!(!tr.contains(&2));
        assert!(tr.contains(&3));
        assert!(tr.contains(&4));
        assert!(!tr.contains(&5));
        assert!(tr.check_rule());
    }
}
