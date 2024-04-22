#version 410

// from assets

uniform float fTime;
uniform float speed;
uniform float scale_red_x;
uniform float scale_green_y;

precision mediump float;
out vec4 out_color;
layout(location=0)in vec2 p;

float rand(float n){
    return fract(sin(n) * 43758.5453123);
}
float rand(vec2 n) { 
    return fract(sin(dot(n, vec2(12.9898, 4.1414))) * 43758.5453);
}

float noise(float p){
    float fl = floor(p);
    float fc = fract(p);
    return mix(rand(fl), rand(fl + 1.0), fc);
}
float noise(vec2 n) {
    const vec2 d = vec2(0.0, 1.0);
    vec2 b = floor(n), f = smoothstep(vec2(0.0), vec2(1.0), fract(n));
    return mix(mix(rand(b), rand(b + d.yx), f.x), mix(rand(b + d.xy), rand(b + d.yy), f.x), f.y);
}
float n1( float x ) {
    #define hash(v) fract(sin(100.0*v)*4375.5453)
    float f = fract(x);
    float p = floor(x);

    f = f*f*(3.0-2.0*f);

    return mix(hash(f),hash(p),f);
}

void main() {
    float time = speed*fTime;
    vec2 n2 = noise2( p + vec2( time*0.03, sin(time*0.032) ));
    float rn = n2.x*5.0;//noise( p.x ); //n1(p.x);
    float gn = n2.y*3.0;//n1(p.y);
    out_color = vec4( sin( time+rn+p.x*scale_red_x ), sin( gn+p.y*scale_green_y ), sin( ( p.x + p.y ) * 3.0 ), 1.0 );
    //out_color = vec4( rn, rn, rn, 1.0 );
}
