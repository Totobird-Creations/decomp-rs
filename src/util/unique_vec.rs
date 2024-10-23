use std::ops::RangeBounds;
use std::vec::{ Drain, IntoIter };
use std::slice::{ Iter, IterMut };
use std::collections::TryReserveError;
use std::mem::replace;


/// Behaves like an ordered [`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html), though in
///   reality is just a [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).
#[derive(Clone)]
pub struct UniqueVec<T> {
    vec : Vec<T>
}

impl<T> UniqueVec<T> {

    /// Constructs a new, empty `UniqueVec<T>`.
    /// 
    /// The vector will not allocate until elements are pushed onto it.
    /// 
    /// Copied from [`Vec::new`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new) and edited.
    pub fn new() -> Self { Self {
        vec : Vec::new()
    } }

    /// Constructs a new, empty `UniqueVec<T>` with at least the specified capacity.
    /// 
    /// The vector will be able to hold at least `capacity` elements without reallocating. This method
    ///   is allowed to allocate for more elements than `capacity`. If `capacity` is 0, the vector will not
    ///   allocate.
    /// 
    /// It is important to note that although the returned vector has the minimum *capacity* specified, the
    ///   vector will have a zero *length*. For an explanation of the difference between length and capacity, see
    ///   [*Capacity and reallocation*](https://doc.rust-lang.org/std/vec/struct.Vec.html#capacity-and-reallocation).
    /// 
    /// If it is important to know the exact allocated capacity of a `UniqueVec`, always use the [`capacity`](Self::capacity)
    ///   method after construction.
    /// 
    /// For `UniqueVec<T>` where `T` is a zero-sized type, there will be no allocation and the capacity will always be
    ///   `usize::MAX`.
    /// 
    /// ##### Panics
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    /// 
    /// Copied from [`Vec::with_capacity`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity) and edited.
    pub fn with_capacity(capacity : usize) -> Self { Self {
        vec : Vec::with_capacity(capacity)
    } }

}

impl<T> UniqueVec<T> {

    /// Returns the number of elements the vector can hold without reallocating.
    /// 
    /// Copied from [`Vec::capacity`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.capacity).
    pub fn capacity(&self) -> usize { self.vec.capacity() }

    /// Returns an iterator over the slice.
    /// 
    /// The iterator yields all items from start to end.
    /// 
    /// Copied from [`Vec::iter`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter).
    pub fn iter(&self) -> impl Iterator<Item = &T> { self.vec.iter() }

    /// Returns the number of elements in the vector, also referred to as its ‘length’.
    /// 
    /// Copied from [`Vec::len`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.len).
    pub fn len(&self) -> usize { self.vec.len() }

    /// Returns `true` if the vector contains no elements.
    /// 
    /// Copied from [`Vec::is_empty`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.is_empty).
    pub fn is_empty(&self) -> bool { self.vec.is_empty() }

    /// Removes the specified range from the vector in bulk, returning all removed elements as an iterator.
    ///   If the iterator is dropped before being fully consumed, it drops the remaining removed elements.
    /// 
    /// The returned iterator keeps a mutable borrow on the vector to optimize its implementation.
    /// 
    /// ##### Panics
    /// Panics if the starting point is greater than the end point or if the end point is greater than the
    ///   length of the vector.
    /// 
    /// ##### Leaking
    /// If the returned iterator goes out of scope without being dropped (due to
    ///   [`mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html), for example), the vector may have
    ///   lost and leaked elements arbitrarily, including elements outside the range.
    /// 
    /// Copied from [`Vec::drain`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain).
    pub fn drain<R : RangeBounds<usize>>(&mut self, range : R) -> Drain<T> { self.vec.drain(range) }

    // extract_if

    /// Retains only the elements specified by the predicate.
    /// 
    /// In other words, remove all elements `e` for which `f(&e)` returns `false`. This method operates in place,
    ///   visiting each element exactly once in the original order, and preserves the order of the retained elements.
    /// 
    /// Copied from [`Vec::retain`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain).
    pub fn retain<F : FnMut(&T) -> bool>(&mut self, f : F) -> () { self.vec.retain(f) }

    /// Clears the vector, removing all values.
    /// 
    /// Note that this method has no effect on the allocated capacity of the vector.
    /// 
    /// Copied from [`Vec::clear`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.clear).
    pub fn clear(&mut self) -> () { self.vec.clear() }

    /// Reserves capacity for at least `additional` more elements to be inserted in the given `Vec<T>`. The
    ///   collection may reserve more space to speculatively avoid frequent reallocations. After calling `reserve`,
    ///   capacity will be greater than or equal to `self.len() + additional`. Does nothing if capacity is already
    ///   sufficient.
    /// 
    /// ##### Panics
    /// Panics if the new capacity exceeds `isize::MAX` *bytes*.
    /// 
    /// Copied from [`Vec::reserve`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reserve).
    pub fn reserve(&mut self, additional : usize) -> () { self.vec.reserve(additional) }

    /// Reserves the minimum capacity for at least `additional` more elements to be inserted in the given `Vec<T>`.
    ///   Unlike [`reserve`](Self::reserve), this will not deliberately over-allocate to speculatively avoid
    ///   frequent allocations. After calling `reserve_exact`, capacity will be greater than or equal to
    ///   `self.len() + additional`. Does nothing if the capacity is already sufficient.
    /// 
    /// Note that the allocator may give the collection more space than it requests. Therefore, capacity can not be
    ///   relied upon to be precisely minimal. Prefer [`reserve`](Self::reserve) if future insertions are expected.
    /// 
    /// ##### Panics
    /// Panics if the new capacity exceeds `isize::MAX` *bytes*.
    /// 
    /// Copied from [`Vec::reserve_exact`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reserve_exact).
    pub fn reserve_exact(&mut self, additional : usize) -> () { self.vec.reserve_exact(additional) }

    /// Tries to reserve capacity for at least `additional` more elements to be inserted in the given `Vec<T>`.
    ///   The collection may reserve more space to speculatively avoid frequent reallocations. After calling
    ///   `try_reserve`, capacity will be greater than or equal to `self.len() + additional` if it returns `Ok(())`.
    ///   Does nothing if capacity is already sufficient. This method preserves the contents even if an error occurs.
    /// 
    /// ##### Errors
    /// If the capacity overflows, or the allocator reports a failure, then an error is returned.
    /// 
    /// Copied from [`Vec::try_reserve`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve).
    pub fn try_reserve(&mut self, additional : usize) -> Result<(), TryReserveError> { self.vec.try_reserve(additional) }

    /// Tries to reserve the minimum capacity for at least `additional` elements to be inserted in the given `Vec<T>`.
    ///   Unlike [`try_reserve`](Self::try_reserve), this will not deliberately over-allocate to speculatively avoid
    ///   frequent allocations. After calling `try_reserve_exact`, capacity will be greater than or equal to
    ///   `self.len() + additional` if it returns `Ok(())`. Does nothing if the capacity is already sufficient.
    /// 
    /// Note that the allocator may give the collection more space than it requests. Therefore, capacity can not be
    ///   relied upon to be precisely minimal. Prefer [`try_reserve`](Self::try_reserve) if future insertions are expected.
    /// 
    /// Errors
    /// If the capacity overflows, or the allocator reports a failure, then an error is returned.
    /// 
    /// Copied from [`Vec::try_reserve_exact`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve_exact).
    pub fn try_reserve_exact(&mut self, additional : usize) -> Result<(), TryReserveError> { self.vec.try_reserve_exact(additional) }

    /// Shrinks the capacity of the vector as much as possible.
    /// 
    /// The behavior of this method depends on the allocator, which may either shrink the vector in-place or reallocate.
    ///   The resulting vector might still have some excess capacity, just as is the case for
    ///   [`with_capacity`](Self::with_capacity). See
    ///   [Allocator::shrink](https://doc.rust-lang.org/std/alloc/trait.Allocator.html#method.shrink) for more details.
    /// 
    /// Copied from [`Vec::shrink_to_fit`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.shrink_to_fit).
    pub fn shrink_to_fit(&mut self) -> () { self.vec.shrink_to_fit() }

    /// Shrinks the capacity of the vector with a lower bound.
    /// 
    /// The capacity will remain at least as large as both the length and the supplied value.
    /// 
    /// If the current capacity is less than the lower limit, this is a no-op.
    /// 
    /// Copied from [`Vec::shrink_to`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.shrink_to).
    pub fn shrink_to(&mut self, min_capacity : usize) -> () { self.vec.shrink_to(min_capacity) }

    pub fn get(&self, index : usize) -> Option<&T> { self.vec.get(index) }

}

impl<T : Eq> UniqueVec<T> {

    /// Returns `true` if the slice contains an element with the given value.
    /// 
    /// This operation is *O(n)*.
    /// 
    /// Copied from [`Vec::contains`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.contains) and edited.
    pub fn contains(&self, x : &T) -> bool { self.vec.contains(x) }

    /// Adds a value to the vector.
    /// 
    /// Returns whether the value was newly inserted. That is:
    /// - If the vector did not previously contain this value, `true` is returned.
    /// - If the vector already contained this value, `false` is returned, and the vector is not modified: original value is
    ///     not replaced, and the value passed as argument is dropped.
    /// 
    /// Copied from [`HashSet::insert`](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.insert) and edited.
    pub fn insert(&mut self, x : T) -> bool {
        if (self.contains(&x)) { false }
        else {
            self.vec.push(x);
            true
        }
    }

    /// Adds a value to the vector, replacing the existing value, if any, that is equal to the given one. Returns the
    ///   replaced value.
    /// 
    /// Copied from [`HashSet::replace`](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.replace) and edited.
    pub fn replace(&mut self, x : T) -> Option<T> {
        let index = self.vec.iter().position(|y| y == &x);
        if let Some(index) = index {
            Some(replace(&mut self.vec[index], x))
        } else { None }
    }

    /// Removes a value from the vector. Returns whether the value was present in the vector.
    /// 
    /// The value may be any borrowed form of the vector’s value type, but [Eq](https://doc.rust-lang.org/std/hash/trait.Hash.html)
    ///   on the borrowed form *must* match those for the value type.
    /// 
    /// Copied from [`HashSet::remove`](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.remove) and edited.
    pub fn remove(&mut self, x : &T) -> bool {
        let index = self.vec.iter().position(|y| y == x);
        if let Some(index) = index {
            self.vec.remove(index);
            true
        } else { false }
    }

    /// Removes and returns a value in the vector, if any, that is equal to the given one.
    /// 
    /// The value may be any borrowed form of the vector’s value type, but [Eq](https://doc.rust-lang.org/std/hash/trait.Hash.html)
    ///   on the borrowed form *must* match those for the value type.
    /// 
    /// Copied from [`HashSet::take`](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.take) and edited.
    pub fn take(&mut self, x : &T) -> Option<T> {
        let index = self.vec.iter().position(|y| y == x);
        index.map(|i| self.vec.remove(i))
    }

}


impl<T> IntoIterator for UniqueVec<T> {
    type Item     = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<'l, T> IntoIterator for &'l UniqueVec<T> {
    type Item     = &'l T;
    type IntoIter = Iter<'l, T>;
    fn into_iter(self) -> Self::IntoIter {
        (&self.vec).into_iter()
    }
}

impl<'l, T> IntoIterator for &'l mut UniqueVec<T> {
    type Item     = &'l mut T;
    type IntoIter = IterMut<'l, T>;
    fn into_iter(self) -> Self::IntoIter {
        (&mut self.vec).into_iter()
    }
}
