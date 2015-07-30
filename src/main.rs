#![feature(alloc)]

pub struct LinkedCircularBuffer<T> {
	length: usize,
	current_item: Rawlink<Node<T>>,
	direction: Direction,
}

struct Rawlink<T> {
	p: *mut T,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction{Forward, Backward}

impl<T> Copy for Rawlink<T> {}
unsafe impl<T:Send> Send for Rawlink<T> {}
unsafe impl<T:Sync> Sync for Rawlink<T> {}

struct Node<T> {
	next: Rawlink<Node<T>>,
	prev: Rawlink<Node<T>>,
	value: T,
}

impl<T> Rawlink<T> {
	fn none() -> Rawlink<T> {
		Rawlink{p: std::ptr::null_mut()}
	}
	
	fn some(n: &mut T) -> Rawlink<T> {
		Rawlink{p: n}
	}
	
	fn resolve_immut<'a>(&self) -> Option<&'a T> {
		if self.p.is_null() {
			None
		} else {
			Some(unsafe { std::mem::transmute(self.p) })
		}
	}
	
	fn resolve<'a>(&mut self) -> Option<&'a mut T> {
		if self.p.is_null() {
			None
		} else {
			Some(unsafe { std::mem::transmute(self.p) })
		}
	}
	
	fn take(&mut self) -> Rawlink<T> {
		std::mem::replace(self, Rawlink::none())
	}
	
	fn is_none(&self) -> bool {
		self.p.is_null()
	}
}

impl<T> Clone for Rawlink<T> {
	#[inline]
	fn clone(&self) -> Rawlink<T> {
		Rawlink{p: self.p}
	}
}

impl<T> Node<T> {
	fn new(v: T) -> Node<T> {
		Node{value: v, next: Rawlink::none(), prev: Rawlink::none()}
	}
}

impl<T> LinkedCircularBuffer<T> {
	#[inline]
	fn push_forward_node(&mut self, mut new_node: Box<Node<T>>) {
		if self.current_item.is_none() { println!("None"); }
		match self.current_item.resolve() {
			None => {
				if self.current_item.is_none() { println!("Yes None"); }
				new_node.next = Rawlink::some(&mut *new_node);
				new_node.prev = Rawlink::some(&mut *new_node);
				self.current_item = Rawlink::some(&mut *new_node);
			}
			Some(ref mut curr) => {
				// let prev_node = curr.prev.resolve().unwrap();
				// new_node.next = Rawlink::some(curr);
				// new_node.prev = Rawlink::some(prev_node);
				// prev_node.next = Rawlink::some(&mut *new_node);
				// curr.prev = Rawlink::some(&mut *new_node);
			}
		}
		self.length += 1;
	}
	
	// #[inline]
	// fn pop_forward_node(&mut self) -> Option<Box<Node<T>>> {
		// self.current_item.resolve().map(|mut curr| {
			// self.length -= 1;
			// match curr.next.take() {
				// Some(node) => {
					// curr.prev.next = node;
					// node.prev = curr.prev;
				// }
				// None => {}
			// }
			// curr
		// })
	// }
	
	#[inline]
	fn push_backward_node(&mut self, mut new_node: Box<Node<T>>) {
		// match self.current_item.resolve() {
			// None => {
				// new_node.next = link_with_prev(new_node, Rawlink.some(&mut *new_node));
				// self.current_item = Rawlink::some(&mut *new_node);
			// }
			// Some(ref mut curr) => {
				// new_node.next = curr;
				// curr.prev.next = link_with_prev(new_node, curr.prev);
				// new_node.next.prev = new_node;
				// curr = new_node;
			// }
		// }
		// self.length += 1;
	}
}

impl<T> LinkedCircularBuffer<T> {
	#[inline]
	pub fn new() -> LinkedCircularBuffer<T> {
		LinkedCircularBuffer{current_item: Rawlink::none(), length: 0, direction: Direction::Forward}
	}
	
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.current_item.is_none()
	}
	
	#[inline]
	pub fn len(&self) -> usize {
		self.length
	}
	
	#[inline]
	pub fn get_direction(&self) -> Direction {
		self.direction
	}
	
	#[inline]
	pub fn set_direction(&mut self, d: Direction) {
		self.direction = d;
	}
	
	#[inline]
	pub fn clear(&mut self) {
		*self = LinkedCircularBuffer::new()
	}
	
	#[inline]
	pub fn current(&self) -> Option<&T> {
		self.current_item.resolve_immut().as_ref().map(|curr| &curr.value)
	}
	
	pub fn push(&mut self, elt: T) {
		match self.direction {
			Direction::Forward => self.push_forward_node(Box::new(Node::new(elt))),
			Direction::Backward => self.push_backward_node(Box::new(Node::new(elt))),
		}
	}
}

impl<A: std::fmt::Debug> std::fmt::Debug for LinkedCircularBuffer<A> {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		fmt.write_fmt(format_args!("LinkedCircularBuffer {{ length: {}, direction: {:?}, elements: {:?} }}",
			self.length, self.direction, self.current_item))
	}
}

impl<A: std::fmt::Debug> std::fmt::Debug for Rawlink<A> {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(fmt, "Rawlink -> {:?}", self.resolve_immut())
	}
}

impl<A: std::fmt::Debug> std::fmt::Debug for Node<A> {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(fmt, "Node {{ {:?} }}", self.value)
	}
}

use std::cell::UnsafeCell;

struct N {
	pub other: UnsafeCell<*mut N>,
}

impl Drop for N {
	fn drop(&mut self) {
		println!("Dropped");
	}
}

fn main() {
	let mut a;

	{
		let n = Box::new(N{other: UnsafeCell::new(std::ptr::null_mut())});
		let nn = unsafe{std::boxed::into_raw(n)};
		unsafe{*(*nn).other.get() = nn};
		println!("n created");
		//let mut m = N{other: Some(Box::new(n))};
		//println!("m created");
		//n.other = Some(Box::new(n));
		//println!("n and m connected");
		a = nn;
	}
	println!("Dropped?");
	unsafe{Box::from_raw(a)};

	// let mut buff = LinkedCircularBuffer::<i32>::new();
	// assert!(buff.is_empty());
	// assert_eq!(buff.len(), 0);
	// assert_eq!(buff.current(), None);
	// assert_eq!(buff.get_direction(), Direction::Forward);
    // println!("{:?}", buff);
	
	// buff.push(1);
    // println!("{:?}", buff);
	// assert_eq!(buff.len(), 1);
	// assert!(!buff.is_empty());
	// assert_eq!(buff.current(), Some(&1i32));
	
	// buff.push(2);
	// println!("{:?}", buff);
	// assert_eq!(buff.len(), 2);
	// assert_eq!(buff.current(), Some(&1i32));
	
	// buff.set_direction(Direction::Backward);
	// assert_eq!(buff.get_direction(), Direction::Backward);
	// println!("{:?}", buff);
	
	// buff.clear();
	// assert_eq!(buff.len(), 0);
	// assert!(buff.is_empty());
	
    // println!("{:?}", buff);
	
	// for i in 0..1024 {
		// for j in 0..1024 {
			// let mut l = LinkedCircularBuffer::<i32>::new();
			// for k in 0..1024 {
				// l.push(i*j + k);
			// }
		// }
	// }
}
