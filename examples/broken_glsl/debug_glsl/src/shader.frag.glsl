#version 450
layout(binding = 0) uniform vec2 screen_size;

layout(location = 0) in vec4 percent_position;
layout(location = 0) out vec4 fragColor;

/*
TODO
    wgpu error: Validation Error
    Caused by:
        In Device::create_shader_module
    Shader validation error:
      ┌─ :1:1
      │
    1 │
      │   naga::Type [1]
        Function [1] 'main' is invalid
        Local variable [1] 'my_unique_var' is invalid
        Initializer doesn't match the variable type
*/


void main() {
    vec4 my_unique_var = vec4(0.5, 0.5, 0.5, 1.0);
    fragColor = my_unique_var;
}