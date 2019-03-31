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

pub trait ResourceLoader<T> {
    /// Loads a resource using the given resource file path
    fn load(&mut self, resource_file_path: &'static str) -> Result<T, String>;
}

pub trait ResourceStore<T> {
    /// Adds a resource to the ResourceStore
    fn store(&mut self, resource_file_path: &'static str, value: T);
    /// Removes a resource from the ResourceStore
    fn remove(&mut self, resource_file_path: &'static str);

    /// Returns a resource as a ref
    fn get(&self, resource_file_path: &'static str) -> Option<&T>;
    /// Returns a resource as a mut ref
    fn get_mut(&mut self, resource_file_path: &'static str) -> Option<&mut T>;
}
