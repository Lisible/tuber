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


use std::rc::Rc;
use std::cell::RefCell;

use std::any::Any;
use game::GameState;

enum Event {
    InputDown(&'static str),
    InputUp(&'static str),
    CloseRequest,
    PushStateRequest(Box<GameState>),
    PopStateRequest,
    Custom(&'static str, Box<Any>),
}

trait EventListener {
    fn on_event(&mut self, event: &Event);
}

struct EventDispatcher {
    listeners: Vec<Rc<RefCell<EventListener>>>
}

impl EventDispatcher {
    pub fn new() -> EventDispatcher {
        EventDispatcher {
            listeners: vec!()
        }
    }

    pub fn register_listener(&mut self,
                             listener: Rc<RefCell<EventListener>>) {
        self.listeners.push(listener);
    }

    pub fn dispatch(&mut self, event: Event) {
        self.listeners
            .iter_mut()
            .for_each(|l| l.borrow_mut().on_event(&event));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct A {
        a: i32
    }

    impl EventListener for A {
        fn on_event(&mut self, e: &Event) {
            match *e {
                Event::InputDown("inc") => self.a += 1,
                Event::InputDown("dec") => self.a -= 1,
                _ => {}
            }
        }
    }

    #[test]
    fn can_register_listener() {
        let mut dispatcher = EventDispatcher::new();
        let a1 = Rc::new(RefCell::new(A{a: 0}));
        let a2 = Rc::new(RefCell::new(A{a: 0}));
        dispatcher.register_listener(a1.clone());
        dispatcher.register_listener(a2.clone());

        assert_eq!(dispatcher.listeners.len(), 2);
    }

    #[test]
    fn can_dispatch_event() {
        let mut dispatcher = EventDispatcher::new();
        let a1 = Rc::new(RefCell::new(A{a: 0}));
        let a2 = Rc::new(RefCell::new(A{a: 0}));
        dispatcher.register_listener(a1.clone());
        dispatcher.register_listener(a2.clone());

        dispatcher.dispatch(Event::InputDown("inc"));
        dispatcher.dispatch(Event::InputDown("inc"));
        dispatcher.dispatch(Event::InputDown("inc"));

        assert_eq!(a1.borrow().a, 3);
        assert_eq!(a2.borrow().a, 3);
    }
}


