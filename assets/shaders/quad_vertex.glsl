#version 100

// vertex position
attribute vec3 position;
// the uv position
attribute vec2 texcoord;
// vertex color data
attribute vec4 color0;
// time passing around

// Transformation matrices 
uniform mat4 Model;
uniform mat4 Projection;

uniform vec4 _Time;

varying lowp vec2 uv;
varying float iTime;

void main() {
    // covert position from world space to projection
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;

};
