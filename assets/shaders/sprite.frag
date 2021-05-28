#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform ColorMaterial_color {
    vec4 Color;
};

layout(set = 2, binding = 2) uniform MousePos_value {
    vec3 mouse_pos;
};


# ifdef COLORMATERIAL_TEXTURE 
layout(set = 1, binding = 1) uniform texture2D ColorMaterial_texture;
layout(set = 1, binding = 2) uniform sampler ColorMaterial_texture_sampler;
# endif

void main() {
    // Light stuff
    // Init vars
    float color_change = 0.0;

    // Light settings
    float light_radius = 200.0;
    float max_light_intensity = 0.75;

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
    color_change = mix(max_light_intensity, 0.0, mouse_vert_dist);


    vec4 color = Color;
# ifdef COLORMATERIAL_TEXTURE
    color *= texture(
        sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),
        v_Uv);
# endif
    // Don't mess with the transparent part of the sprites
    if (color.a != 0) {
        color.a += 0.3;

    } else {
        // White background
        //color = vec4(1.0, 1.0, 1.0, 1.0);

    }
    o_Target = color * color_change;
}
