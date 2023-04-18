#[cfg(test)]
mod tests {
    use crate::tree::Tree;
    use crate::import::str2tree;

    #[test]
    fn treemake() {
        let ts = String::from("4(2(3)(1))(6(5))");
        let tree = str2tree(ts, String::from("Tree1"));
        
        assert_eq!(tree.get_node(0).unwrap().parent, None);
        assert_eq!(tree.get_node(4).unwrap().parent, Some(0));
        assert_eq!(tree.get_parent(1).unwrap().children, (Some(1), Some(4)));
        assert_eq!(tree.get_root().unwrap().parent, None);

        
    }

    #[test]
    fn iteratetree() {
        let ts = String::from("1(2(5(6))(4))(3)");
        let tree = str2tree(ts, String::from("Tree1"));


        assert_eq!(tree.iter(tree.get_node(3)).fold(0,|acc, _node| acc + 1), 4);
        assert_eq!(tree.iter(tree.get_root()).fold(0,|acc, _node| acc + 1), 1);

        assert_eq!(tree.leftiter(tree.get_root()).fold(0,|acc, _node| acc + 1), 6);
    }
}