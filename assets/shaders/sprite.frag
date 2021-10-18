#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform ColorMaterial_color {
    vec4 Color;
};

layout(set = 2, binding = 2) uniform ShaderMousePosition_value {
    vec2 mouse_pos;
};

layout(set = 2, binding = 3) uniform HelmetColor_value {
    vec3 helmet_color;
};

layout(set = 2, binding = 4) uniform InnerSuitColor_value {
    vec3 inner_suit_color;
};

layout(set = 2, binding = 5) uniform WindowSize_value {
    vec2 screen_dimensions;
};

layout(set = 2, binding = 6) uniform Alpha_value {
    float phasing;
};


# ifdef COLORMATERIAL_TEXTURE 
layout(set = 1, binding = 1) uniform texture2D ColorMaterial_texture;
layout(set = 1, binding = 2) uniform sampler ColorMaterial_texture_sampler;
# endif

//Lighting settings
const float light_radius = 300.0;
const float max_light_intensity = 1.0;

// Converts a color from sRGB gamma to linear light gamma
vec4 color_encode(vec4 color) {
    float r = color.r < 0.04045 ? (1.0 / 12.92) * color.r : pow((color.r + 0.055) * (1.0 / 1.055), 2.4);
    float g = color.g < 0.04045 ? (1.0 / 12.92) * color.g : pow((color.g + 0.055) * (1.0 / 1.055), 2.4);
    float b = color.b < 0.04045 ? (1.0 / 12.92) * color.b : pow((color.b + 0.055) * (1.0 / 1.055), 2.4);

    return vec4(r, g, b, color.a);
}


// Light math
void add_lighting(inout vec4 color) {
    // Init vars
    float color_change = 0.0;    

    vec2 pixel_pos = gl_FragCoord.xy / screen_dimensions;
    vec2 mouse_position = mouse_pos;
    //mouse_position.y = screen_dimensions.y - mouse_position.y;

    // By diving the distance of the mouse and the pixel, we can shrink how much is lit by the light
    //mouse_position /= light_radius;
    //pixel_pos /= light_radius;

    float mouse_vert_dist = 0.0;

    mouse_vert_dist = distance(mouse_position, pixel_pos);
    color_change = mix(max_light_intensity, 0.0, mouse_vert_dist);

    color *= color_change;

}

const float f32_epsilon = 0.00000011920929;

void set_color_of_player(inout vec4 color) {
    // Set color of player parts
    // By default, different body parts of players will be different colors, and the shader just looks for body parts by finding their color.

    // Helmet
    if ( abs(color.r - 0.392) <= 0.384 ) {
        color = vec4(helmet_color, color.a);

    // Suit
    } else if ( abs(color.r - 0.078) <= 0.08 && abs(color.g - 0.392) <= 0.36 && abs(color.b - 0.392) <= 0.36 ) {
        color = vec4(inner_suit_color, color.a);

    }

}

void main() {
    vec4 color = Color;

    color.a = phasing;

    // Don't mess with the transparent part of the sprites
    # ifdef COLORMATERIAL_TEXTURE
        color *= texture(sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler), v_Uv);
    # endif
    set_color_of_player(color);
    //add_lighting(color);

    o_Target = color;
}
