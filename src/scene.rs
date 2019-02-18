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
    pub fn new() -> SceneGraph {
        SceneGraph {
            root_node: Box::new(SceneNode::new("root", NodeValue::AbstractNode))
        }
    }
}

pub enum NodeValue {
    AbstractNode,
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
    

    /// Finds a successor in the tree using an identifier
    ///
    /// # Examples
    ///
    /// ```
    /// use tuber::scene::{SceneNode, NodeValue};
    ///
    /// let mut root = SceneNode::new("root", NodeValue::AbstractNode);
    /// let mut a = SceneNode::new("a", NodeValue::AbstractNode);
    /// let b = SceneNode::new("b", NodeValue::AbstractNode);
    /// let c = SceneNode::new("c", NodeValue::AbstractNode);
    /// 
    /// a.add_child(c);
    /// root.add_child(a);
    /// root.add_child(b);
    ///
    ///
    /// match root.find("c") {
    ///     Some(..) => assert!(true),
    ///     _ => assert!(false)
    /// }
    ///
    /// match root.find("a").unwrap().find("c") {
    ///     Some(..) => assert!(true),
    ///     _ => assert!(false)
    /// }
    ///
    /// match root.find("d") {
    ///     None => assert!(true),
    ///     _ => assert!(false)
    /// }
    ///
    /// ```
    pub fn find(&self, identifier: &'static str) -> Option<&SceneNode> {
        use std::collections::HashSet;

        let mut stack: Vec<&SceneNode> = vec!(self);
        let mut visited = HashSet::new();

        while stack.len() != 0 {
            if let Some(vertex) = stack.pop() {
                
                if vertex.identifier == identifier {
                    return Some(vertex);
                }

                if !visited.contains(vertex.identifier) {
                    visited.insert(vertex.identifier);
                    for child in &vertex.children {
                        stack.push(&child);
                    }
                }
            }
        }

        None
    }
}