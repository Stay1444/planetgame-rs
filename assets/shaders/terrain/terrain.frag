#version 450

layout(set = 2, binding = 0) uniform vec4 color;

layout(location = 0 ) out vec4 o_Target;

void main() {
    o_Target = color;
  }
