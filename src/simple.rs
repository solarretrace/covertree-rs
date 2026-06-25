// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Simple (non-parallel) cover tree implementation.
//!
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use Point;
use DEFAULT_SPAN_FACTOR;


// Standard library imports.
use std::default;
use std::mem;


////////////////////////////////////////////////////////////////////////////////
// CoverTree
////////////////////////////////////////////////////////////////////////////////
/// A cover tree containing [`Point`]s of type P.
///
/// [`Point`]: trait.Point.html
#[derive(Debug, Clone, PartialEq)]
pub struct CoverTree<P> where P: Point {
    /// The root of the tree.
    root: Option<Cover<P>>,
    /// The span factor for each Cover.
    span_factor: f64,
    /// The number of items in the tree.
    len: usize
}


impl<P> CoverTree<P> where P: Point {
    /// Constructs an empty `CoverTree`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// #    
    /// let cover_tree: CoverTree<f32> = CoverTree::new();
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn new() -> CoverTree<P> {
        Default::default()
    }

    /// Constructs an empty `CoverTree` with the specified span factor.
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// #    
    /// let cover_tree: CoverTree<f32> = CoverTree::with_span_factor(2.0);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn with_span_factor(span_factor: f64) -> Self {
        CoverTree {
            root: None,
            span_factor: span_factor,
            len: 0,
        }
    }

    /// Constructs a `CoverTree` containing all of the [`Point`]s in the given 
    /// [`Iterator`].
    ///
    /// [`Point`]: trait.Point.html
    /// [`Iterator`]: http://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    /// 
    /// let cover_tree: CoverTree<f32> = CoverTree::from_items(nums.into_iter());
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn from_items<I>(points: I) -> Self where I: Iterator<Item=P> {
        let mut tree = CoverTree::new();
        tree.insert_all(points);
        tree
    }

    // TODO(Sky): Can we avoid mut here?
    /// Returns the point nearest to the given of [`Point`] in the `CoverTree`.
    ///
    /// [`Point`]: trait.Point.html
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    /// 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::from_items(nums.into_iter());
    /// 
    /// let nearest = cover_tree.find_nearest(1.2).unwrap();
    /// assert_eq!(nearest, &1.3f32);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn find_nearest<'a>(&'a mut self, query_point: P) -> Option<&'a P> {
        if let Some(ref mut cover) = self.root {
            Some(cover.find_nearest(query_point, None))
        } else {
            None
        }
    }

    /// Returns the number of [`Point`]s in the `CoverTree`.
    ///
    /// [`Point`]: trait.Point.html
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    /// 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::from_items(nums.into_iter());
    /// 
    /// assert_eq!(cover_tree.len(), 4);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the span factor of the `CoverTree`.
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::with_span_factor(2.1);
    /// 
    /// assert_eq!(cover_tree.span_factor(), 2.1f64);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn span_factor(&self) -> f64 {
        self.span_factor
    }

    /// Returns `true` if the `CoverTree` is empty.
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    /// 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::from_items(nums.into_iter());
    /// 
    /// assert!(!cover_tree.is_empty());
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Inserts the given [`Point`] into the `CoverTree`.
    ///
    /// [`Point`]: trait.Point.html
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::new();
    ///
    /// cover_tree.insert(1.3);
    /// cover_tree.insert(1.52);
    /// 
    /// assert_eq!(cover_tree.len(), 2);
    /// assert_eq!(cover_tree.find_nearest(1.4).unwrap(), &1.3f32);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn insert(&mut self, point: P) {
        let new_cover = Cover::new(point, 1);
        if let Some(ref mut cover) = self.root {
            let n = mem::replace(cover, new_cover);
            mem::replace(cover, n.insert(point, self.span_factor));
        } else {
            self.root = Some(new_cover);
        }

        self.len += 1;
    }

    /// Inserts each of the [`Point`]s in the given [`Iterator`] into the 
    /// `CoverTree`.
    ///
    /// [`Point`]: trait.Point.html
    /// [`Iterator`]: http://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::new();
    ///
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    ///
    /// cover_tree.insert_all(nums.into_iter());
    /// 
    /// assert_eq!(cover_tree.len(), 4);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn insert_all<I>(&mut self, points: I) where I: Iterator<Item=P> {
        for point in points {
            self.insert(point);
        }
    }

    /// Removes the given [`Point`] from the `CoverTree`.
    ///
    /// [`Point`]: trait.Point.html
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    /// 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::from_items(nums.into_iter());
    /// 
    /// assert_eq!(cover_tree.len(), 4);
    ///
    /// cover_tree.remove(1.3);
    ///
    /// assert_eq!(cover_tree.len(), 3);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn remove(&mut self, point: P) -> Option<P> {
        if let Some(ref mut cover) = self.root {
            let removed = cover.remove(point);
            if removed.is_some() {self.len -= 1;}
            removed
        } else {
            None
        }
    }

    /// Removes each [`Point`] in the given [`Iterator`] from the `CoverTree`.
    ///
    /// [`Point`]: trait.Point.html
    /// [`Iterator`]: http://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    /// let mut cover_tree = CoverTree::from_items(nums.into_iter());
    /// 
    /// assert_eq!(cover_tree.len(), 4);
    ///
    /// let unwanted = vec![1.0, 2.3, 4.6];
    /// cover_tree.remove_all(unwanted.into_iter());
    ///
    /// assert_eq!(cover_tree.len(), 2);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn remove_all<I>(&mut self, points: I) where I: Iterator<Item=P> {
        for point in points {
            self.remove(point);
        }
    }

    /// Removes all points from the `CoverTree`.
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    /// 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::from_items(nums.into_iter());
    /// 
    /// cover_tree.clear();
    /// assert!(cover_tree.is_empty());
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn clear(&mut self) {
        self.root = None;
        self.len = 0;
    }
}


impl<P> default::Default for CoverTree<P> where P: Point {
    fn default() -> Self {
        CoverTree::with_span_factor(DEFAULT_SPAN_FACTOR)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cover
////////////////////////////////////////////////////////////////////////////////
/// A node in a `CoverTree` containing a point of type P.
#[derive(Debug, Clone, PartialEq)]
struct Cover<P> where P: Point {
    /// The point stored in the `Cover`.
    point: P,
    /// The children of the `Cover`. Each child must be withing cover_distance
    /// of the point.
    children: Vec<Cover<P>>,
    /// The level of the `Cover`.
    level: usize,
    /// The maximum distance from the point to any of its descendents.
    max_distance: Option<f64>
}


impl<P> Cover<P> where P: Point {
    /// Constructs a new `Cover` with the given point and level.
    pub fn new(point: P, level: usize) -> Cover<P> {
        Cover {
            point, 
            children: Vec::new(), 
            level,
            max_distance: None
        }
    }

    /// Returns the size of the cover.
    fn cover_distance(&self, span_factor: f64) -> f64 {
        span_factor.powf(self.level as f64)
    }

    /// Calculates the maximum distance between the `Cover`s point and any of
    /// its children's points. Ignores any cached value.
    fn max_distance_(&self) -> f64 {
        let mut dist = 0.0;
        for descendent in self.descendents() {
            dist = self.point
                .distance(descendent.point)
                .max(dist);
        }
        dist
    }

    /// Returns the maximum distance between the `Cover`s point and any of its
    /// children's points.
    fn max_distance(&mut self) -> f64 {
        if let Some(dist) = self.max_distance {
            dist
        } else {
            let dist = self.max_distance_();
            self.max_distance = Some(dist);
            dist
        }
    }

    /// Returns all of the `Cover`s under this one.
    fn descendents(&self) -> Vec<&Cover<P>> {
        let mut descendents: Vec<&Cover<P>> = Vec::new();
        
        for child in &self.children {
            descendents.push(&child);
            for desc in child.descendents() {
                descendents.push(desc);
            }
        }
        descendents
    }

    /// Adds a new child `Cover` to this node.
    fn add_child(&mut self, cover: Cover<P>) {
        self.children.push(cover);
    }

    // Pseudocode from paper:
    // function findNearestNeighbor(Point tree p, 
    //                              query point x, 
    //                              nearest neighbor so far y)
    //     if P(p, x) < P(y, x) then
    //         y←p
    //     for each child q of p sorted by distance to x do
    //         if P(y, x) > P(x, q) − maxdist(q) then
    //             y ← findNearestNeighbor(q, x, y)
    //     return y 
    pub fn find_nearest<'a>(&'a mut self, 
                        query: P,
                        nearest_yet: Option<&'a P>) 
                        -> &'a P {
        
        // Save closes value yet seen.
        let mut nearest = if nearest_yet.is_none() || self.point.distance(query) < nearest_yet
            .expect("point is nearest yet")
            .distance(query) 
        { 
            &self.point 
        } else {
            nearest_yet.expect("provided is nearest yet")
        };

        // Sort children by distance to query point.
        self.children.sort_by(|a: &Cover<P>, b: &Cover<P>| 
            a.point
                .distance(query)
                .partial_cmp(&b.point.distance(query))
                .expect("sort by distance to target")
        );
        
        for child in &mut self.children {
            // If closer points could be below this one, recurse.
            if nearest.distance(query) > query.distance(child.point) - child.max_distance() {
                nearest = child.find_nearest(query, Some(&nearest));
            }
        }
        nearest
    }

    // Pseudocode from paper:
    // function insert(Point tree p, point point x) 
    //     if P(p, x) > covdist(p) then
    //         while P(p, x) > 2*covdist(p) do
    //             Remove any leaf q from p
    //             p′ ← tree with root q and p as only child
    //             p ← p′
    //         return tree with x as root and p as only child
    //     return insert_(p, x)
    fn insert(mut self,
              point: P,
              span_factor: f64) ->Cover<P> {

        // Cache the maximum distance for this Cover.
        self.max_distance.map(|x| x.max(self.point.distance(point)));

        if self.point.distance(point) > self.cover_distance(span_factor) {
            while self.point.distance(point) > self.cover_distance(span_factor) * 2.0 {
                self.promote_leaf();
            }
            let mut root = Cover::new(point, self.level + 1);
            root.children = vec![self];
            return root;
        }

        self.insert_(point, span_factor)
    }

    // Pseudocode from paper:
    // function insert_(Point tree p, point point x)
    //     prerequisites: P(p,x) ≤ covdist(p)
    //     for q ∈ children(p) do
    //          if P(q, x) ≤ covdist(q) then
    //              q′ ← insert_(q, x)
    //              p′ ← p with child q replaced with q′
    //              return p′
    //     return p with x added as a child 
    fn insert_(mut self,
               point: P,
               span_factor: f64) -> Cover<P> {
        
        // Verify that the Cover can be inserted here.
        let dist = self.point.distance(point);
        let covdist = self.cover_distance(span_factor);
        assert!(dist <= covdist,
                "CoverTree invariant violated: P(p,x) ≤ covdist(p)"); 

        // Cache the maximum distance for this Cover.
        self.max_distance.map(|x| x.max(self.point.distance(point)));

        let mut done = false;
        
        for child in self.children.iter_mut() {
            let dummy = Cover::new(point, 0); // Placeholder point.
            if child.point.distance(point) <= child.cover_distance(span_factor) {

                // Gain ownership over child and insert point.
                let child_new = mem::replace(child, dummy)
                                  .insert_(point, span_factor);

                // Restore child to where it was.
                mem::replace(child, child_new);

                // We want to return self, but we've borrowed children, 
                // so we just set a flag and break instead.
                done = true;
                break;
            }
        }

        if !done {
            // No children: just add the one we've got.
            if self.level == 1 {self.level += 1;}
            let new_cover = Cover::new(point, self.level-1);
            self.add_child(new_cover);
        }
        self
    }

    pub fn remove(&mut self, query: P) -> Option<P> {
        let mut removed = None;
        let mut was_last = false;

        // Sort children by distance to query point.
        self.children.sort_by(|a: &Cover<P>, b: &Cover<P>|
            a.point
                .distance(query)
                .partial_cmp(&b.point.distance(query))
                .expect("sort by distance to target")
        );
        
        if let Some(index) = self.children
                               .iter()
                               .position(|x| x.point == query) {
            // Remove leaf and set was_last flag if needed.
            removed = Some(self.children.swap_remove(index).point);
            if self.children.len() == 0 {was_last = true;}
        } else {
            for child in &mut self.children {
                removed = child.remove(query);
                if removed.is_some() {break;}
            }
        }
        
        if was_last {self.children = Vec::new();} // Erase empty Vec.
        if removed.is_some() {self.max_distance = None;} // Clear cache.
        removed
    }

    // Remove any leaf q from p
    // p′ ← tree with root q and p as only child
    // p ← p′
    fn promote_leaf(&mut self) {
        if self.children.is_empty() {
            self.level += 1;
        } else if let Some(leaf) = self.remove_leaf() {
            let old_root = mem::replace(self, leaf);
            self.level = old_root.level + 1;
            self.add_child(old_root);
        }
    }

    fn remove_leaf(&mut self) -> Option<Cover<P>> {
        // Find index of leaf.
        if let Some(index) = self.children
            .iter()
            .position(|x| x.children.is_empty())
        {
            // Remove leaf and set was_last flag if needed.
            Some(self.children.swap_remove(index))
        } else {
            // There are no leaves at this level, so recurse.
            self.children
                .first_mut()
                .expect("get first child")
                .remove_leaf();
            None
        }
    }
}
