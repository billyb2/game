#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform ColorMaterial_color {
    vec4 Color;
};

layout(set = 2, binding = 2) uniform MousePosition_value {
    vec3 mouse_pos;
};

layout(set = 2, binding = 3) uniform HelmetColor_value {
    vec3 helmet_color;
};

layout(set = 2, binding = 4) uniform InnerSuitColor_value {
    vec3 inner_suit_color;
};


# ifdef COLORMATERIAL_TEXTURE 
layout(set = 1, binding = 1) uniform texture2D ColorMaterial_texture;
layout(set = 1, binding = 2) uniform sampler ColorMaterial_texture_sampler;
# endif

//Lighting settings
const float light_radius = 300.0;
const float max_light_intensity = 1.0;

//const vec4 default_helmet_color = vec4(96.0, 96.0, 96.0, 1.0);

// Converts a color from sRGB gamma to linear light gamma
vec4 toLinear(vec4 sRGB) {
    bvec4 cutoff = lessThan(sRGB, vec4(0.04045));
    vec4 higher = pow((sRGB + vec4(0.055))/vec4(1.055), vec4(2.4));
    vec4 lower = sRGB/vec4(12.92);

    return mix(higher, lower, cutoff);
}


// Light math
void add_lighting(inout vec4 color) {
    // Init vars
    float color_change = 0.0;    

    vec2 pixel_pos = gl_FragCoord.xy;
    vec2 mouse_position = mouse_pos.xy;

    // By diving the distance of the mouse and the pixel, we can shrink how much is lit by the light
    mouse_position /= light_radius;
    pixel_pos /= light_radius;

    float mouse_vert_dist = 0.0;

    mouse_vert_dist = distance(mouse_position, pixel_pos);
    color_change = mix(max_light_intensity, 0.0, mouse_vert_dist);

    color *= color_change;

}

void set_color_of_player(inout vec4 color) {
    // Set color of player parts
    // By default, different body parts of players will be different colors, and the shader just looks for body parts by finding their color.
    // Body
    if (color.x >= 70.0 / 255.0) {
        color = vec4(inner_suit_color, 1.0);

    // Helmet
    } else if (color.x >= 15.0 / 255.0) {
        color = vec4(helmet_color, 1.0);

    // Reflection on helmet
    } else if (color.y >= 13.0 / 255.0) {
        color = vec4(helmet_color * 0.9, 1.0);

    // Pants
    } else if (color.x >= 13.0 / 255.0) {
        color = vec4(67.0 / 255.0, 42.0 / 255.0, 42.0 / 255.0, 1.0);

    }
}

void main() {
    vec4 color = Color;
# ifdef COLORMATERIAL_TEXTURE
    color *= texture(sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),v_Uv);
# endif
    // Don't mess with the transparent part of the sprites
    if (color.a != 0) {
        set_color_of_player(color);
        //add_lighting(color);


    }

    o_Target = toLinear(color);
}
