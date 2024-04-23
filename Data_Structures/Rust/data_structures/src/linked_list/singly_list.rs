#[derive(Debug)]
struct SinglyListNode<T> {
    pub data: T,
    pub next: Option<Box<SinglyListNode<T>>>,
}

impl<T> SinglyListNode<T> {
    fn new(data: T) -> SinglyListNode<T> {
        Self {
            data: data,
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct Head<T> {
    head: Option<Box<SinglyListNode<T>>>,
}

impl<T: PartialEq> Head<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn add_last(&mut self, data: T) {
        let new_element = Some(Box::new(SinglyListNode::new(data)));

        match self.head.as_mut() {
            None => {
                self.head = new_element;
            }
            Some(mut temp_element) => {
                while let Some(ref mut next_node) = temp_element.next {
                    temp_element = next_node;
                }
                temp_element.next = new_element;
            }
        }
    }

    pub fn get_index(&mut self, index: i32) -> Option<&T> {
        if index < 0 {
            return None;
        }

        match self.head.as_mut() {
            None => None,
            Some(mut temp_element) => {
                let mut i: i32 = 0;
                while i < index {
                    temp_element = temp_element.next.as_mut().unwrap();
                    i += 1;
                }
                Some(&temp_element.data)
            }
        }
    }

    pub fn get_lenght(&mut self) -> i32 {
        match self.head.take() {
            None => 0,
            Some(mut element) => {
                let mut lenght = 1;
                while let Some(next) = element.next {
                    element = next;
                    lenght += 1;
                }
                lenght
            }
        }
    }

    // pub fn delete_element(&mut self, delete_data: T) -> bool {
    //     let mut current = temp_element;
    //     while let Some(ref mut node) = current.next {
    //         if current.data.is_some() && current.data.unwrap() == delete_data {
    //             if current.data == self.head.unwrap().data {
    //                 self.head = self.head.unwrap().next;
    //                 return true;
    //             }

    //             if temp_element.data.is_some() {
    //                 temp_element.next = current.next;
    //                 return true;
    //             }
    //         }
    //     }

    //     true
    // }

    pub fn delete(&mut self, delete: T) {
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            match &node.data {
                data => {
                    if *data == delete {
                        self.head.unwrap().data = self.head.unwrap().next.unwrap().data;
                    }
                }
            }
            current = &mut node.next;
        }
    }
}

#[cfg(test)]
mod test_singly_list {
    use super::*;

    #[test]
    fn basics_i32() {
        let mut list: Head<i32> = Head::new();
        list.add_last(5);
        list.add_last(10);
        list.add_last(11);
        assert_eq!(list.get_index(0), Some(&5));
        assert_eq!(list.get_index(1), Some(&10));
        assert_eq!(list.get_index(2), Some(&11));
    }

    #[test]
    fn basics_string() {
        let mut list: Head<String> = Head::new();

        list.add_last("ABC".to_string());
        list.add_last("123".to_string());
        list.add_last("Teste".to_string());

        assert_eq!(list.get_index(0), Some(&"ABC".to_string()));
        assert_eq!(list.get_index(1), Some(&"123".to_string()));
        assert_eq!(list.get_index(2), Some(&"Teste".to_string()));
    }

    #[test]
    fn get_lenght() {
        let mut list = Head::new();
        list.add_last(5);
        list.add_last(6);
        list.add_last(7);
        list.add_last(8);
        assert_eq!(list.get_lenght(), 2);
    }
}
