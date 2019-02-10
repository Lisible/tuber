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

use window::Window;

pub trait GameState {
    fn initialize(&mut self){}
    fn update(&mut self){}
}

pub struct Game {
    state_stack: Vec<Box<dyn GameState>>,
    window: Box<dyn Window>,
}

impl Game {
    pub fn new(initial_state: Box<dyn GameState>, window: Box<dyn Window>) -> Game {
        Game {
            state_stack: vec!(initial_state),
            window,
        }
    }

    /// Starts the main loop of the game
    pub fn run(&mut self) {
        println!("Game started");

        'main_loop: loop {
            if self.state_stack.is_empty() { break 'main_loop; }

            let current_state = self.state_stack.last_mut().expect("State stack is empty");
            while let Some(_input) = self.window.poll_input() {
                //current_state.handle_input(input);
            }

            current_state.update();
        }

        println!("Game stopped");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use window::Headless;

    struct DummyState;
    impl GameState for DummyState {}

    struct IncrementState {
        value: u32
    }

    impl GameState for IncrementState {
        fn update(&mut self) {
            self.value += 1;
        }
    }

    #[test]
    fn new_game() {
        let game = Game::new(Box::new(DummyState {}), Box::new(Headless::new()));
        assert_eq!(game.state_stack.len(), 1);
    }

    #[test]
    fn update_game_state() {
        let mut game_state = IncrementState { value: 0 };
        game_state.update();
        assert_eq!(game_state.value, 1);
        game_state.update();
        assert_eq!(game_state.value, 2);
        game_state.update();
        assert_eq!(game_state.value, 3);
    }
}
