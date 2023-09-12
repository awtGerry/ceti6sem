use std::os::raw::c_void;

use gl::types::*;

/* -- Usar buffers de opengl 3.3+ -- */

pub struct VAO {
    id: gl::types::GLuint,
}

impl VAO {
    pub fn new() -> VAO {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VAO { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

/* -- Creacion de un nuevo buffer -- */
pub struct BufferGL {
    id: gl::types::GLuint,
    r#type: gl::types::GLenum,
    usage: gl::types::GLenum,
}

impl BufferGL {
    pub fn new(r#type: gl::types::GLenum, usage: gl::types::GLenum) -> BufferGL {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        BufferGL { id, r#type, usage }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, 0);
        }
    }

    pub fn buffer_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                self.usage,
            );
        }
    }
}

/* -- Creacion de un nuevo vertex array -- */
pub struct VertexGL {
    index: GLuint,
}

impl VertexGL {
    pub fn new(
        index: GLuint,
        size: GLint,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> VertexGL {
        unsafe {
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer);
            gl::EnableVertexAttribArray(index);
        }
        VertexGL { index }
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}
