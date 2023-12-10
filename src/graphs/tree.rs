use std::{collections::VecDeque, usize};

#[derive(Debug)]
struct TreeNode{
    adjacency_list:Vec<u32>,
    shortest_path:Option<Vec<u32>>
}

#[inline]
fn idx(n:u32)->usize{
    usize::try_from(n).unwrap()
}

pub struct Tree{
    nodes:Vec<TreeNode>
}
impl Tree{
    pub fn from_total_nodes(n:&u32)->Self{
        let mut v:Vec<TreeNode> = Vec::new();
        for _ in 0..*n{
            v.push(TreeNode {adjacency_list: Vec::new(),shortest_path: None})
        }
        let s=Tree{nodes:v};
        s
    }

    pub fn make_edge(&mut self,n1:u32,n2:u32){
        self.nodes[usize::try_from(n1).unwrap()].adjacency_list.push(n2);
        self.nodes[usize::try_from(n2).unwrap()].adjacency_list.push(n1);
    }

    /// Finds out all the previous nodes that are to be visited for node n to get by shortest path
    /// 
    /// (Since this is a tree, only path is shortest path)
    pub fn create_shortest_paths(&mut self,n:u32)->&Vec<u32>{
        let mut v=Vec::new();
        let mut qu:VecDeque<u32>=VecDeque::new();

        v.reserve_exact(self.nodes.len());v.fill(0);
        v[idx(n)]=n;
        qu.push_back(n);

        loop {
            match qu.pop_front(){
                None=> break,
                Some(n)=>for adj in &self.nodes[idx(n)].adjacency_list{
                    if v[idx(*adj)]==0{v[idx(*adj)]=n;qu.push_back(n)}
                }
            }
        }

        let dest =&mut self.nodes[idx(n)].shortest_path;
        *dest=Some(v);
        dest.as_ref().unwrap()
    }

    /// returns a vec of nodes which are all interconnected starting from n1 to n2
    /// 
    /// Creates a path if there are none.
    pub fn path(&mut self,n1:u32,n2:u32)->Vec<u32>{
        match &self.nodes[idx(n1)].shortest_path {
            Some(v)=>{
                let mut v1= Vec::new();
                let mut current=n1;
                while current!=n2 {
                    let next=v[idx(current)];
                    v1.push(current);
                    current=next;
                }
                v1
            },
            None=>
            match &self.nodes[idx(n2)].shortest_path{
                Some(v)=>{
                    let mut v =  v.clone();
                    v.reverse();v
                },
                None=>{self.create_shortest_paths(n1);self.path(n1,n2)}
            }
        }
    }
}