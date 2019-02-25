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

use std::ffi::{CString, CStr};

pub struct ShaderProgram {
    id: gl::types::GLuint
}

impl ShaderProgram {
    pub fn from_shaders(shaders: &[Shader]) -> Result<ShaderProgram, String> {
        let id = shader_program_from_shaders(shaders)?;

        Ok(ShaderProgram {
            id
        })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn use_shader(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    id: gl::types::GLuint
}

impl Shader {
    pub fn from_source(source: &CStr, 
                       shader_type: gl::types::GLenum
    ) -> Result<Shader, String> {
        let id = shader_from_source(source, shader_type)?;

        Ok(Shader {
            id
        })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr,
                      shader_type: gl::types::GLenum
) -> Result<gl::types::GLuint, String> {
    let shader_id = unsafe { gl::CreateShader(gl::VERTEX_SHADER) }; 
    unsafe {
        gl::ShaderSource(shader_id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader_id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut info_log_length: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut info_log_length);
        }

        let error = create_whitespace_cstring_with_length(info_log_length as usize); 

        unsafe {
            gl::GetShaderInfoLog(
                shader_id,
                info_log_length,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );

            return Err(error.to_string_lossy().into_owned());
        }
    }

    Ok(shader_id)
}

fn shader_program_from_shaders(shaders: &[Shader]) -> Result<gl::types::GLuint, String> {
    let program_id = unsafe { gl::CreateProgram() };

    for shader in shaders {
        unsafe {
            gl::AttachShader(program_id, shader.id());
        }
    }

    unsafe { gl::LinkProgram(program_id); }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
    }

    if success == 0 {
        let mut info_log_length: gl::types::GLint = 0;
        unsafe {
            gl::GetProgramiv(program_id, 
                             gl::INFO_LOG_LENGTH, 
                             &mut info_log_length);
        }

        let error = create_whitespace_cstring_with_length(info_log_length as usize);

        unsafe {
            gl::GetProgramInfoLog(
                program_id,
                info_log_length,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    for shader in shaders {
        unsafe {
            gl::DetachShader(program_id, shader.id());
        }
    }

    Ok(program_id)
}



fn create_whitespace_cstring_with_length(length: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(length as usize + 1);
    buffer.extend([b' '].iter().cycle().take(length as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}
