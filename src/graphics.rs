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

pub mod scene_renderer;

pub type Point = (f32, f32, f32);
pub type Color = (f32, f32, f32, f32);

/// Struct for text
pub struct Text {
    text: String,
}

/// Struct for sprites
pub struct Sprite {
    width: f32,
    height: f32,
    texture_identifier: String,
}

impl Sprite {
    /// Creates a new sprite with the given size and texture
    pub fn new(width: f32, height: f32, texture_identifier: String) -> Sprite {
        Sprite {
            width,
            height,
            texture_identifier,
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn texture_identifier(&self) -> &str {
        &self.texture_identifier
    }
}

pub struct Line {
    first_point: Point,
    second_point: Point,
    color: Color,
}

impl Line {
    pub fn new(first_point: Point, second_point: Point, color: Color) -> Line {
        Line {
            first_point,
            second_point,
            color,
        }
    }

    pub fn first_point(&self) -> Point {
        self.first_point
    }

    pub fn second_point(&self) -> Point {
        self.second_point
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

pub struct Rectangle {
    width: f32,
    height: f32,
    color: Color,
}

impl Rectangle {
    pub fn new(width: f32, height: f32, color: Color) -> Rectangle {
        Rectangle {
            width,
            height,
            color,
        }
    }

    /// Returns the width of the rectangle
    pub fn width(&self) -> f32 {
        self.width
    }
    /// Returns the height of the rectangle
    pub fn height(&self) -> f32 {
        self.height
    }
    /// Returns the size (width, height) of the rectangle
    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    /// Returns the color of the rectangle
    pub fn color(&self) -> &Color {
        &self.color
    }
}
