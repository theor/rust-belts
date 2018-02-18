pub struct QuadTree<'a, T> {
    capacity: u32,
    depth: u32,
    max_depth: u32,
    bounds: Bounds,
    elements: Vec<T>,
    children: Option<[Box<QuadTree<'a, T>>; 4]>,
}

/// A bounded area represented by x, y, width, and height.
#[derive(PartialEq, Eq, Debug)]
pub struct Bounds {
    /// x coordinate
    pub x: i32,
    /// y coordinate
    pub y: i32,
    /// width
    pub width: i32,
    /// height
    pub height: i32
}

pub trait Bounded {
    /// Returns a bounded area.
    fn bounds(&self) -> Bounds;
}

#[derive(PartialEq, Eq)]
enum Quadrant { TL, TR, BR, BL }

impl<'a, T: Bounded> QuadTree<'a, T> {
    /// Constructs a new quadtree containing the specified bounds.
    pub fn new(bounds: Bounds) -> QuadTree<'a, T> {
        QuadTree {
            capacity: 4,
            max_depth: 10,
            depth: 0,
            bounds: bounds,
            elements: Vec::new(),
            children: None
        }
    }

    // quadrant:
    //   children:
    //     children[quadrant].push
    //   no children:
    //     self.push
    //     split
    // no quadrant:
    //    children:
    //      self.push
    //    no children:
    //      self.push
    //      split
    /// Inserts an element into the quadtree.
    pub fn insert(&mut self, element: &'a T) {
        match (self.get_quadrant(element), self) {
            (Some(q), &QuadTree{children: Some(ref mut children), .. }) => {
                children[q as u32].insert(element);
            },
            (None, _self@&QuadTree{children: Some(_), .. }) => {
                _self.elements.push(element);
            },
            (_, _self@&QuadTree{children: None, .. }) => {
                _self.elements.push(element);

                if _self.elements.len() > _self.capacity {
                    _self.split();
                }
            },
        };
    }

    /// Returns an iterator over elements near a given element, which may or may not be in the quadtree.
    pub fn query(&'a self, element: &'a T) -> QueryItems<'a, T> {
        QueryItems{
            qt: self,
            index: 0,
            element: element,
            next_qts: Vec::new()
        }
    }

    /// Returns an iterator over all elements in the quadtree.
    pub fn iter(&'a self) -> Items<'a, T> {
        Items{
            root: self,
            quadrants: Vec::new(),
            element_index: 0
        }
    }

    fn split(&mut self) {
        if self.depth >= self.max_depth { return; }

        match self.children {
            Some(_) => unreachable!(),
            None => {
                let mut children = [
                    Box::new(QuadTree {
                        capacity: self.capacity,
                        depth: self.depth + 1,
                        max_depth: self.max_depth,
                        bounds: Bounds{x: self.bounds.x,
                                       y: self.bounds.y,
                                       width: self.bounds.width / 2.0,
                                       height: self.bounds.height / 2.0 },
                        elements: Vec::new(),
                        children: None}),
                    Box::new(QuadTree{
                        capacity: self.capacity,
                        depth: self.depth + 1,
                        max_depth: self.max_depth,
                        bounds: Bounds{x: self.bounds.x + self.bounds.width / 2.0,
                                       y: self.bounds.y,
                                       width: self.bounds.width / 2.0,
                                       height: self.bounds.height / 2.0},
                        elements: Vec::new(),
                        children: None}),
                    Box::new(QuadTree{
                        capacity: self.capacity,
                        depth: self.depth + 1,
                        max_depth: self.max_depth,
                        bounds: Bounds{x: self.bounds.x + self.bounds.width / 2.0,
                                       y: self.bounds.y + self.bounds.height / 2.0,
                                       width: self.bounds.width / 2.0,
                                       height: self.bounds.height / 2.0},
                        elements: Vec::new(),
                        children: None}),
                    Box::new(QuadTree{
                        capacity: self.capacity,
                        depth: self.depth + 1,
                        max_depth: self.max_depth,
                        bounds: Bounds{x: self.bounds.x,
                                       y: self.bounds.y + self.bounds.height / 2.0,
                                       width: self.bounds.width / 2.0,
                                       height: self.bounds.height / 2.0},
                        elements: Vec::new(),
                        children: None})
                    ];

                let mut new_elements: Vec<&T> = Vec::new();
                for &element in self.elements.iter() {
                    match self.get_quadrant(element) {
                        Some(i) => children[i as u32].insert(element),
                        None => new_elements.push(element)
                    };
                }

                self.children = Some(children);
                self.elements = new_elements;
            }
        }
    }

    fn get_quadrant(&self, r: &T) -> Option<Quadrant> {
        let half_width = self.bounds.x + (self.bounds.width / 2.0);
        let half_height = self.bounds.y + (self.bounds.height / 2.0);

        let fits_left_half = r.bounds().x >= self.bounds.x &&
            r.bounds().x + r.bounds().width < half_width;
        let fits_right_half = r.bounds().x >= half_width &&
            r.bounds().x + r.bounds().width < self.bounds.x + self.bounds.width;
        let fits_top_half = r.bounds().y >= self.bounds.y &&
            r.bounds().y + r.bounds().height < half_height;
        let fits_bottom_half = r.bounds().y >= half_height &&
            r.bounds().y + r.bounds().height < self.bounds.y + self.bounds.height;

        if fits_top_half && fits_left_half { Some(Quadrant::TL) }
        else if fits_top_half && fits_right_half { Some(Quadrant::TR) }
        else if fits_bottom_half && fits_right_half { Some(Quadrant::BR) }
        else if fits_bottom_half && fits_left_half { Some(Quadrant::BL) }
        else { None }
    }

    fn contains(&self, r: &T) -> bool {
        r.bounds().x >= self.bounds.x && r.bounds().x + r.bounds().width < self.bounds.width &&
            r.bounds().y >= self.bounds.y && r.bounds().y + r.bounds().height < self.bounds.height
    }

// }

// impl<'a, T: Bounded> Container for QuadTree<'a, T> {
    fn len(&self) -> usize {
        let mut count = self.elements.len();
        match self.children {
            Some(ref children) => {
                for child in children.iter() { count += child.len(); }
            },
            None => {}
        };
        count
    }
    fn clear(&mut self) {
        self.elements.clear();
        match self {
            _self@&QuadTree{ children: Some(_), .. } => {
                match _self.children {
                    Some(ref mut children) => {
                        for child in children.mut_iter() {
                            child.clear();
                        }
                    },
                    None => {}
                }
                _self.children = None;
            },
            _ => {}
        }
    }
}
use std::fmt;
impl<'a, T: Bounded + fmt::Display> fmt::Display for QuadTree<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for _ in 0..self.depth { s.push_str("    ") }

        s.push_str(format!("{}", self.elements));

        match self.children {
            Some(ref children) => {
                for child in children.iter() {
                    s.push_str("\n");
                    s.push_str(child.to_str());
                }
            },
            None => {}
        };

        write!(f.buf, "{}", s)
    }
}