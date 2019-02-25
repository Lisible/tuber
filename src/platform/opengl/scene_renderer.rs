/*
* MIT License
*
* Copyright (c) 2018-2019 Clément SIBILLE
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

use crate::scene::{SceneNode, SceneGraph, NodeValue};
use crate::graphics::scene_renderer;
use crate::platform::opengl::low_level_renderer::LowLevelRenderer;

pub struct SceneRenderer {
    low_level_renderer: LowLevelRenderer
}

impl SceneRenderer {
    pub fn new() -> SceneRenderer {
        SceneRenderer {
            low_level_renderer: LowLevelRenderer::new()
        }
    }
}

impl scene_renderer::SceneRenderer for SceneRenderer {
    fn render_scene(&mut self, scene: &SceneGraph) {
        use std::collections::HashSet;

        let root = scene.root();
        let mut stack = vec!(root);
        let mut visited = HashSet::new();

        while stack.len() != 0 {
            if let Some(node) = stack.pop() {
               
                // TODO change this
                self.low_level_renderer.render_triangle();

                if !visited.contains(node.identifier()) { 
                    visited.insert(node.identifier());
                    for child in node.children() {
                        stack.push(child);
                    }
                }
            }
        }
    }
}
