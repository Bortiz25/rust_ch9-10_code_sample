#[derive(Clone)]
#[derive(Debug)]
enum Node<T> {
    Node(Box<List<T>>),
    Nil,
}

#[derive(Clone)]
#[derive(Debug)]
struct List<T> {
    val: T,
    next: Node<T>,
}

impl<T: std::fmt::Debug + std::cmp::PartialEq> List<T> {
    fn append(&mut self, elem: T) {
        match self.next {
            Node::Node(ref mut next_node) => {
                println!("Element {:?} is being pushed", elem);
                next_node.append(elem);
            }
            Node::Nil => {
                let new_node: List<T> = List {
                    val: elem,
                    next: Node::Nil,
                };
                self.next = Node::Node(Box::new(new_node));
            }
        }
    }

    fn len(&self) -> u32 {
        match self.next {
            Node::Node(ref new_node) => 1 + new_node.len(),
            Node::Nil => 0,
        }
    }

    fn delete(&mut self, elem: T) {
        match self.next {
            Node::Node(ref mut new_node) => {
                panic!("Value is present but we lack the capability to delete item");
            }
            Node::Nil => {
                if self.val == elem {
                    panic!("Value present but unable to delete item")
                } else {
                    println!("Element {:?} does not exist", elem)
                }
            }
        }
    }
}
fn main() {
    let mut str_l = List {
        val: "Hello",
        next: Node::Nil,
    };
    str_l.append("world");
    str_l.append("elem");
    str_l.append("hello");

    println!("The size of the str_l is {}", str_l.len());

    let mut l = List {
        val: 1,
        next: Node::Nil,
    };
    l.append(2);
    l.append(3);
    l.append(4);

    l.delete(2);

    println!("The size of l is {}", l.len());
}
