#version 450

layout(location = 0) out vec4 percent_position;
void main() {
    // Full screen triangle
    float expand = 3.0;
    float x = float(gl_VertexIndex - 1);
    float y = float((gl_VertexIndex & 1) * 2 - 1);
    gl_Position = vec4(expand * x, expand * y, 0.0, 1.0);
    percent_position = vec4(0.5 * (1.0 + (expand * x)), 0.5 * (1.0 - (expand * y)), 0.0, 1.0);
}
