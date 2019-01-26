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

pub enum Input {
    /// Window close
    Close,
    /// Window resize (width, height)
    Resize(u32, u32),
    /// Keyboard key down
    KeyDown(keyboard::Key),
    /// Keyboard key up
    KeyUp(keyboard::Key),
    /// Mouse button down
    MouseDown(mouse::Button),
    /// Mouse button move
    MouseUp(mouse::Button),
    /// Mouse wheel event
    MouseWheelChange(u32, i32),
    None
}

/// Trait for input listeners
pub trait InputListener {
    fn on_close(&mut self){}
    fn on_resize(&mut self, _width: u32, _height: u32){}
    fn on_key_down(&mut self, _key: keyboard::Key){}
    fn on_key_up(&mut self, _key: keyboard::Key){}
    fn on_mouse_down(&mut self, _button: mouse::Button){}
    fn on_mouse_up(&mut self, _button: mouse::Button){}
    fn on_mouse_wheel_change(&mut self, _absolute: u32, _relative: i32){}
}


pub mod keyboard {
    pub enum Key {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
        Return,
        Enter,
        LShift,
        RShift,
        LControl,
        RControl,
        Escape,
        Unknown
    }
}

pub mod mouse {
    pub enum Button {
        Left,
        Right,
        Middle,
        Unknown
    }
}
