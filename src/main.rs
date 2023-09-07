use partition_list::ListNode;


mod partition_list;

fn make_nodes(values: &Vec<i32>) -> Option<Box<partition_list::ListNode>> {
    let mut head = None;
    let mut tail: &mut Option<Box<partition_list::ListNode>> = &mut None;
    for val in values {
        let node = Box::new(partition_list::ListNode::new(*val));
        if let Some(tail_node) = tail {
            tail_node.next = Some(node); 
            tail = &mut tail_node.next;
        } else {
            head = Some(node);
            tail = &mut head;
        }
    }
    return head;
}

fn print_list(mut head: &Option<Box<ListNode>>, delim: &str) -> String {
    let mut print = String::new();
    while let Some(node) = head {
        if print.len() > 0 {
            print.push_str(delim);
        }
        print.push_str(format!("{}", node.val).as_str());

        head = &node.next;
    }
    return print;
}

fn main() {
    let values = vec![1, 4, 3, 2, 5, 2];
    let input = make_nodes(&values);
    let output = partition_list::Solution::partition(input, 3);
    println!("Input: {}", print_list(&make_nodes(&values), &" -> "));
    println!("Output: {}", print_list(&output, &" -> "));
}
