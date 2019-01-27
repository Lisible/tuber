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

use input::Input;

pub trait Window {
    /// Shows the window on the screen
    fn show(&mut self);
    /// Hides the window from the screen
    fn hide(&mut self);

    /// Polls an event from the window's event loop
    fn poll_event(&mut self) -> Input;

    fn set_current_graphics_context(&self);
}

pub struct Headless;

impl Headless {
    pub fn new() -> Headless {
        Headless {}
    }
}

impl Window for Headless {
    fn show(&mut self) {}
    fn hide(&mut self) {}

    fn poll_event(&mut self) -> Input {
        Input::None
    }

    fn set_current_graphics_context(&self) {}
}