#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use winapi::shared::minwindef::HMODULE;
use winapi::um::libloaderapi::LoadLibraryA;
use winapi::um::libloaderapi::GetProcAddress;
use winapi::um::wingdi::wglGetProcAddress;
use core::mem;

pub enum CVoid {}

pub type GLboolean = u8;
pub type GLchar = u8;
pub type GLfloat = f32;
pub type GLenum = u32;
pub type GLint = i32;
pub type GLuint = u32;
pub type GLsizei = i32;
pub type GLsizeiptr = isize;
 
pub const FALSE: GLboolean = 0;
pub const TRIANGLES: GLenum = 0x0004;
pub const TRIANGLE_STRIP: GLenum = 0x0005;
pub const FLOAT: GLenum = 0x1406;
pub const COLOR: GLenum = 0x1800;
pub const FRAGMENT_SHADER: GLenum = 0x8B30;
pub const VERTEX_SHADER: GLenum = 0x8B31;
pub const COMPILE_STATUS: GLenum = 0x8B81;
pub const LINK_STATUS: GLenum = 0x8B82;
pub const ARRAY_BUFFER: GLenum = 0x8892;
pub const STATIC_DRAW: GLenum = 0x88E4;

const AttachShaderIdx: u16 = 2;     // 
const BindBufferIdx: u16 = 8;
const BindVertexArrayIdx: u16 = 26;
const BufferDataIdx: u16 = 40;
const CreateProgramIdx: u16 = 96;
const ClearBufferfvIdx: u16 = 49;
const CompileShaderIdx: u16 = 73;
const CreateShaderIdx: u16 = 101;
const DetachShaderIdx: u16 = 128;
const DrawArraysIdx: u16 = 135;
const EnableVertexAttribArrayIdx: u16 = 157;
const GenBuffersIdx: u16 = 175;
const GenVertexArraysIdx: u16 = 185;
const GetProgramInfoLogIdx: u16 = 254;
const GetProgramivIdx: u16 = 256;
const GetShaderInfoLogIdx: u16 = 280;
const GetShaderivIdx: u16 = 281;
const GetUniformLocationIdx: u16 = 313;
const LinkProgramIdx: u16 = 350;
const ShaderSourceIdx: u16 = 479;
const Uniform1fIdx: u16 = 539;
const UseProgramIdx: u16 = 591;
const VertexAttribPointerIdx: u16 = 682;

const wglSwapIntervalIdx: u16 = 695;

static LOAD_DESC: &'static [(u16, &'static str)] = &[

    ( wglSwapIntervalIdx, "wglSwapIntervalEXT\0" ),
    (DrawArraysIdx, "glDrawArrays\0"),

    // Program functions
    (CreateProgramIdx, "glCreateProgram\0"),
    (GetProgramivIdx, "glGetProgramiv\0"),
    (AttachShaderIdx, "glAttachShader\0"),    
    (DetachShaderIdx, "glDetachShader\0"),
    (UseProgramIdx, "glUseProgram\0"),

    (LinkProgramIdx, "glLinkProgram\0"),
    (ClearBufferfvIdx, "glClearBufferfv\0"),
    (CreateShaderIdx, "glCreateShader\0"),
    (ShaderSourceIdx, "glShaderSource\0"),
    (CompileShaderIdx, "glCompileShader\0"),
    (GetShaderivIdx, "glGetShaderiv\0"),
    (GetShaderInfoLogIdx, "glGetShaderInfoLog\0"),
    (GetProgramInfoLogIdx, "glGetProgramInfoLog\0"),

    (GenVertexArraysIdx, "glGenVertexArrays\0"),
    (BindVertexArrayIdx, "glBindVertexArray\0"),

    (GenBuffersIdx, "glGenBuffers\0"),
    (BindBufferIdx, "glBindBuffer\0"),
    (BufferDataIdx, "glBufferData\0"),
    (EnableVertexAttribArrayIdx, "glEnableVertexAttribArray\0"),
    (VertexAttribPointerIdx, "glVertexAttribPointer\0"),

    (GetUniformLocationIdx, "glGetUniformLocation\0"),
    (Uniform1fIdx, "glUniform1f\0"),
  
//    ( wglSwapIntervalIdx, "wglSwapIntervalEXT\0" ),
    // (CreateProgramIdx, b"glCreateProgram\0"),
    // (ClearBufferfvIdx, b"glClearBufferfv\0"),
];

static mut GL_API: [usize; 696] = [0; 696];

pub unsafe fn wglSwapIntervalEXT(interval: GLint ) -> GLuint {
    mem::transmute::<_, extern "system" fn(GLint) -> GLuint>(*GL_API.get_unchecked(wglSwapIntervalIdx as usize))(interval)
}


pub unsafe fn CreateProgram() -> GLuint {
    mem::transmute::<_, extern "system" fn() -> GLuint>(*GL_API.get_unchecked(CreateProgramIdx as usize))()
}

pub unsafe fn LinkProgram(program: GLuint) -> () {
    mem::transmute::<_, extern "system" fn(GLuint) -> ()>(*GL_API.get_unchecked(LinkProgramIdx as usize))(program)
}

pub unsafe fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) -> () {
    mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(*GL_API.get_unchecked(GetProgramivIdx as usize))(program, pname, params)
}

pub unsafe fn UseProgram(program: GLuint) -> () {
    mem::transmute::<_, extern "system" fn(GLuint) -> ()>(*GL_API.get_unchecked(UseProgramIdx as usize))(program)
}

pub unsafe fn AttachShader(program: GLuint, shader: GLuint) -> () {
    mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(*GL_API.get_unchecked(AttachShaderIdx as usize))(program, shader)
}

pub unsafe fn DetachShader(program: GLuint, shader: GLuint) -> () {
    mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(*GL_API.get_unchecked(DetachShaderIdx as usize))(program, shader)
}

pub unsafe fn CreateShader(type_: GLenum) -> GLuint {
    mem::transmute::<_, extern "system" fn(GLenum) -> GLuint>(*GL_API.get_unchecked(CreateShaderIdx as usize))(type_)
}

pub unsafe fn ClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLfloat) -> ()>(*GL_API.get_unchecked(ClearBufferfvIdx as usize))(buffer, drawbuffer, value)
}

pub unsafe fn ShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) -> () {
    mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint) -> ()>(*GL_API.get_unchecked(ShaderSourceIdx as usize))(shader, count, string, length)
}

pub unsafe fn CompileShader(shader: GLuint) -> () {
    mem::transmute::<_, extern "system" fn(GLuint) -> ()>(*GL_API.get_unchecked(CompileShaderIdx as usize))(shader)
}

pub unsafe fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) -> () {
    mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(*GL_API.get_unchecked(GetShaderivIdx as usize))(shader, pname, params)
}

pub unsafe fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () {
    mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(*GL_API.get_unchecked(GetShaderInfoLogIdx as usize))(shader, bufSize, length, infoLog)
}

pub unsafe fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> () {
    mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(*GL_API.get_unchecked(GetProgramInfoLogIdx as usize))(program, bufSize, length, infoLog)
}

pub unsafe fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint) -> () {
    mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(*GL_API.get_unchecked(GenVertexArraysIdx as usize))(n, arrays)
}

pub unsafe fn BindVertexArray(array: GLuint) -> () {
    mem::transmute::<_, extern "system" fn(GLuint) -> ()>(*GL_API.get_unchecked(BindVertexArrayIdx as usize))(array)
}

pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint) -> () {
    mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(*GL_API.get_unchecked(GenBuffersIdx as usize))(n, buffers)
}

pub unsafe fn BindBuffer(target: GLenum, buffer: GLuint) -> () {
    mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(*GL_API.get_unchecked(BindBufferIdx as usize))(target, buffer)
}

pub unsafe fn BufferData(target: GLenum, size: GLsizeiptr, data: *const CVoid, usage: GLenum) -> () {
    mem::transmute::<_, extern "system" fn(GLenum, GLsizeiptr, *const CVoid, GLenum) -> ()>(*GL_API.get_unchecked(BufferDataIdx as usize))(target, size, data, usage)
}

pub unsafe fn EnableVertexAttribArray(index: GLuint) -> () {
    mem::transmute::<_, extern "system" fn(GLuint) -> ()>(*GL_API.get_unchecked(EnableVertexAttribArrayIdx as usize))(index)
}

pub unsafe fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const CVoid) -> () {
    mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const CVoid) -> ()>(*GL_API.get_unchecked(VertexAttribPointerIdx as usize))(index, size, type_, normalized, stride, pointer)
}

pub unsafe fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei) -> ()>(*GL_API.get_unchecked(DrawArraysIdx as usize))(mode, first, count)
}

pub unsafe fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
    mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(*GL_API.get_unchecked(GetUniformLocationIdx as usize))(program, name)
}

pub unsafe fn Uniform1f(location: GLint, v0: GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(GLint, GLfloat) -> ()>(*GL_API.get_unchecked(Uniform1fIdx as usize))(location, v0)
}

pub fn init() {
    let handle : HMODULE;
    unsafe { handle = LoadLibraryA( "Opengl32.dll\0".as_ptr() as *const i8);  }
    for &(index, name) in LOAD_DESC {
        unsafe {
            let mut prc = wglGetProcAddress(name.as_ptr() as *const i8) as usize;
            if prc == 0 {
                prc = GetProcAddress( handle, name.as_ptr() as *const i8 ) as usize;
            }
            GL_API[ index as usize] =  prc;
        }
    }
}