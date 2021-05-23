#version 450 

layout(set = 3, binding = 0) uniform WindowSize_value {
    vec2 window_size;
};

layout(set = 2, binding = 0) uniform MousePos_value {
    vec3 mouse_pos;
};

in vec4 gl_FragCoord;

layout(location = 0) out vec4 o_Target;

void main() {
    // Init vars
    float color_change = 0.0;

    // The initial color of the object (currently bright red)
    vec3 color = vec3(1.00, 0.0, 0.00);

    // Light settings
    float light_radius = 200.0;
    float light_intensity = 0.4;

    // Ambient light is the light of the entire "scene"
    //float ambient_light = 0.1;

    vec2 pixel_pos = gl_FragCoord.xy;
    vec2 mouse_position = mouse_pos.xy;

    // Adjust the mouse's position slightly (why I have to do this, I'm not sure)
    mouse_position += vec2(50.0, 25.0);


    // By diving the distance of the mouse and the pixel, we can shrink how much is lit by the light
    mouse_position /= light_radius;
    pixel_pos /= light_radius;

    float mouse_vert_dist = 0.0;

    mouse_vert_dist = distance(mouse_position, pixel_pos);
    color_change = mix(light_intensity, 0.0, mouse_vert_dist);

    o_Target = vec4(color * color_change, 1.0);
}
