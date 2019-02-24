/*
 * MIT License
 *
 * Copyright (c) 2018 Clément SIBILLE
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

pub struct SceneGraph {
    root_node: Box<SceneNode>
}

impl SceneGraph {
    /// Creates a scene graph with a abstract root
    pub fn new() -> SceneGraph {
        SceneGraph {
            root_node: Box::new(SceneNode::new("root", NodeValue::AbstractNode))
        }
    }

    /// Returns a reference to the root of the scene graph
    pub fn root(&self) -> &SceneNode {
        &self.root_node
    }

    /// Returns a mutable reference to the root of the scene graph
    pub fn root_mut(&mut self) -> &mut SceneNode {
        &mut self.root_node
    }
}

pub enum NodeValue {
    AbstractNode,
    OpenGLTriangleNode,
}

pub struct SceneNode {
    identifier: &'static str,
    value: NodeValue,
    children: Vec<SceneNode>
}

impl SceneNode {
    pub fn new(identifier: &'static str, value: NodeValue) -> SceneNode {
        SceneNode {
            identifier,
            value,
            children: vec!()
        }
    }

    /// Adds a child to the node
    pub fn add_child(&mut self, node: SceneNode) {
        self.children.push(node); 
    }

    /// Returns the identifier of the node
    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    /// Returns the value of the node
    pub fn value(&self) -> &NodeValue {
        &self.value
    }

    /// Returns the children of the node
    pub fn children(&self) -> &Vec<SceneNode> {
        &self.children
    }
    /// Returns the children of the node mutably
    pub fn children_mut(&mut self) -> &mut Vec<SceneNode> {
        &mut self.children
    }

    /// Finds a successor in the tree using an identifier
    pub fn find(&self, identifier: &'static str) -> Option<&SceneNode> {
        use std::collections::HashSet;

        let mut stack = vec!(self);
        let mut visited = HashSet::new();

        while stack.len() != 0 {
            if let Some(vertex) = stack.pop() {
                if vertex.identifier == identifier {
                    return Some(vertex);
                }

                if !visited.contains(vertex.identifier) {
                    visited.insert(vertex.identifier);
                    for child in &vertex.children {
                        stack.push(child);
                    }
                }
            }
        }

        None
    }
    /// Finds a successor in the tree using an identifier 
    pub fn find_mut(&mut self, identifier: &'static str) -> Option<&mut SceneNode> {
        use std::collections::HashSet;

        let mut stack: Vec<&mut SceneNode> = vec!(self);
        let mut visited = HashSet::new();

        while stack.len() != 0 {
            if let Some(vertex) = stack.pop() {
                
                if vertex.identifier == identifier {
                    return Some(vertex);
                }

                if !visited.contains(vertex.identifier) {
                    visited.insert(vertex.identifier);
                    for child in &mut vertex.children {
                        stack.push(child);
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_graph_new() {
        let scene_graph = SceneGraph::new();
        let root = scene_graph.root();

        match root.value() {
            NodeValue::AbstractNode => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn scene_node_add_child() {
        let mut root = SceneNode::new("root", NodeValue::AbstractNode);
        root.add_child(SceneNode::new("a", NodeValue::AbstractNode));
        root.add_child(SceneNode::new("b", NodeValue::AbstractNode));
        
        assert_eq!(root.children.len(), 2);
    }

    #[test]
    fn scene_node_find() {
        let mut root = SceneNode::new("root", NodeValue::AbstractNode);
        root.add_child(SceneNode::new("a", NodeValue::AbstractNode));
        root.add_child(SceneNode::new("b", NodeValue::AbstractNode));
       
        if let None = root.find("a") {
            assert!(false);
        }

        root.find_mut("b").unwrap().add_child(SceneNode::new("c", NodeValue::AbstractNode));

        if let None = root.find("c") {
            assert!(false);
        }

        if let None = root.find("b").unwrap().find("c") {
            assert!(false);
        }
    }
}
