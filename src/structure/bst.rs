use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self.left.as_ref().unwrap().borrow().tree_search(value);
            } else if self.right.is_some() {
                return self.right.as_ref().unwrap().borrow().tree_search(value);
            }
        }
        //default if current node is NIL
        None
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    pub fn maximum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * NOTE: Buggy from pull request
     * Find node successor according to the book
     * Should return None, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        // directly check if the node has a right child, otherwise go to the next block
        if let Some(right_node) = &x_node.borrow().right {
            return Some(right_node.borrow().minimum());
        } 
        
        // empty right child case
        else { 
            let mut x_node = x_node;
            let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
            let mut temp: BstNodeLink;

            while let Some(ref exist) = y_node {
                if let Some(ref left_child) = exist.borrow().left {
                    if BstNode::is_node_match(left_child, x_node) {
                        return Some(exist.clone());
                    }
                }

                temp = y_node.unwrap();
                x_node = &temp;
                y_node = BstNode::upgrade_weak_to_strong(temp.borrow().parent.clone());
            }

            None    
        }
    }

    /**
     * Alternate simpler version of tree_successor that made use of is_nil checking
     */
    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink>{
        //create a shadow of x_node so it can mutate
        let mut x_node = x_node;
        let right_node = &x_node.borrow().right.clone();
        if BstNode::is_nil(right_node)!=true{
            return Some(right_node.clone().unwrap().borrow().minimum());
        }

        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let y_node_right = &y_node.clone().unwrap().borrow().right.clone();
        let mut y_node2: Rc<RefCell<BstNode>>;
        while BstNode::is_nil(&y_node) && BstNode::is_node_match_option(Some(x_node.clone()), y_node_right.clone()) {
            y_node2 = y_node.clone().unwrap();
            x_node = &y_node2;
            let y_parent = y_node.clone().unwrap().borrow().parent.clone().unwrap();
            y_node = BstNode::upgrade_weak_to_strong(Some(y_parent));
        }

        //in case our sucessor traversal yield root, means self is the highest key
        if BstNode::is_node_match_option(y_node.clone(), Some(BstNode::get_root(&x_node))) {
            return None;
        }

        //default return self / x_node
        return Some(y_node.clone().unwrap())
    }

    /**
     * private function return true if node doesn't has parent nor children nor key
     */
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                {
                    return true;
                }
                return false;
            }
        }
    }

    //helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }
    pub fn tree_insert_link(root: &BstNodeLink, new_node: &BstNodeLink) {
        // Special case: root has no key (dummy root, likely from tree creation)
        if root.borrow().key.is_none() {
            let mut root_mut = root.borrow_mut();
            *root_mut = new_node.borrow().clone();
            return;
        }
    
        let mut x = Some(root.clone());
        let mut y = None;
    
        while let Some(current) = x.clone() {
            let current_key = current.borrow().key;
            let new_key = new_node.borrow().key;
    
            match (new_key, current_key) {
                (Some(nk), Some(ck)) => {
                    y = x.clone();
                    if nk < ck {
                        x = current.borrow().left.clone();
                    } else {
                        x = current.borrow().right.clone();
                    }
                }
                _ => {
                    panic!("Cannot insert: one of the node keys is None");
                }
            }
        }
    
        if let Some(ref parent) = y {
            new_node.borrow_mut().parent = Some(Rc::downgrade(parent));
            let new_key = new_node.borrow().key.unwrap();
            let parent_key = parent.borrow().key.unwrap();
    
            if new_key < parent_key {
                parent.borrow_mut().left = Some(new_node.clone());
            } else {
                parent.borrow_mut().right = Some(new_node.clone());
            }
        } else {
            panic!("Insertion failed: parent node is None");
        }
    }
    

    pub fn transplant(u: &BstNodeLink, v: Option<BstNodeLink>) {
        if let Some(parent_weak) = &u.borrow().parent {
            if let Some(parent) = parent_weak.upgrade() {
                if parent.borrow().left.as_ref().map(|x| Rc::ptr_eq(x, u)).unwrap_or(false) {
                    parent.borrow_mut().left = v.clone();
                } else {
                    parent.borrow_mut().right = v.clone();
                }
                if let Some(ref v_node) = v {
                    v_node.borrow_mut().parent = Some(Rc::downgrade(&parent));
                }
            }
        }
    }

    pub fn tree_delete_link(root: &mut BstNodeLink, z: &BstNodeLink) {
        let is_root = Rc::ptr_eq(root, z);
    
        let z_left = z.borrow().left.clone();
        let z_right = z.borrow().right.clone();
        let z_right_clone = z_right.clone();
    
        if z_left.is_none() {
            if is_root {
                if let Some(new_root) = z_right {
                    let new_val = new_root.borrow().clone();
                    *root.borrow_mut() = new_val;
                } else {
                    root.borrow_mut().key = None;
                    root.borrow_mut().left = None;
                    root.borrow_mut().right = None;
                    root.borrow_mut().parent = None;
                }
            } else {
                BstNode::transplant(z, z_right);
            }
        } else if z_right.is_none() {
            if is_root {
                if let Some(new_root) = z_left {
                    let new_val = new_root.borrow().clone();
                    *root.borrow_mut() = new_val;
                } else {
                    root.borrow_mut().key = None;
                    root.borrow_mut().left = None;
                    root.borrow_mut().right = None;
                    root.borrow_mut().parent = None;
                }
            } else {
                BstNode::transplant(z, z_left);
            }
        } else {
            let right_child = z_right.unwrap();
            let successor = right_child.borrow().minimum();
    
            let successor_is_direct_child = Rc::ptr_eq(&successor, &right_child);
            let successor_right = successor.borrow().right.clone();
            let z_left_clone = z_left.clone();
    
            if !successor_is_direct_child {
                BstNode::transplant(&successor, successor_right.clone());
    
                if let Some(ref r) = z_right_clone {
                    successor.borrow_mut().right = Some(r.clone());
                    r.borrow_mut().parent = Some(Rc::downgrade(&successor));
                }
            }
    
            if is_root {
                let succ_clone = successor.borrow().clone();
                *root.borrow_mut() = succ_clone;
    
                if let Some(ref l) = z_left_clone {
                    root.borrow_mut().left = Some(l.clone());
                    l.borrow_mut().parent = Some(Rc::downgrade(root));
                }
    
                if let Some(ref r) = z_right_clone {
                    if !Rc::ptr_eq(r, &successor) {
                        root.borrow_mut().right = Some(r.clone());
                        r.borrow_mut().parent = Some(Rc::downgrade(root));
                    }
                }
            } else {
                BstNode::transplant(z, Some(successor.clone()));
    
                if let Some(ref l) = z_left_clone {
                    successor.borrow_mut().left = Some(l.clone());
                    l.borrow_mut().parent = Some(Rc::downgrade(&successor));
                }
            }
        }
    }
    
    
    
    pub fn tree_search_link(node: &BstNodeLink, value: &i32) -> Option<BstNodeLink> {
        let mut current = node.clone();
    
        loop {
            let next = {
                let current_borrow = current.borrow();
    
                match current_borrow.key {
                    Some(key) if *value == key => return Some(current.clone()),
                    Some(key) if *value < key => current_borrow.left.clone(),
                    Some(_) => current_borrow.right.clone(),
                    None => return None,
                }
            };
    
            match next {
                Some(next_node) => current = next_node,
                None => return None,
            }
        }
    }
    
    
    pub fn tree_insert(root: &mut BstNodeLink, key: i32) {
        let new_node = BstNode::new_bst_nodelink(key);
        BstNode::tree_insert_link(root, &new_node);
    }
    
    pub fn tree_delete(root: &mut BstNodeLink, key: i32) {
        let node_to_delete = Self::tree_search_link(root, &key);
        if let Some(node) = node_to_delete {
            BstNode::tree_delete_link(root, &node);
        } else {
            println!("Key {} not found, cannot delete", key);
        }
    }

} 
