#version 300 es
precision highp float;

in vec2 v_Uv;
out vec4 o_Target;

layout(std140) uniform ColorMaterial_color {
    vec4 Color;
};

layout(std140) uniform ShaderMousePosition_value { // set = 2, binding = 2
    vec2 mouse_pos;
};

layout(std140) uniform HelmetColor_value { // set = 2, binding = 3
    vec3 helmet_color;
};

layout(std140) uniform InnerSuitColor_value { // set = 2, binding = 4
    vec3 inner_suit_color;
};

layout(std140) uniform WindowSize_value { // set = 2, binding = 5
    vec2 screen_dimensions;
};

layout(std140) uniform ShaderPhasing_value {
    float phasing;
};




# ifdef COLORMATERIAL_TEXTURE 
uniform sampler2D ColorMaterial_texture;  // set = 1, binding = 1
# endif

//Lighting settings
const float light_radius = 300.0;
const float max_light_intensity = 1.0;

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
        color = vec4(helmet_color * 0.8, 1.0);

    // Pants
    } else if (color.x >= 13.0 / 255.0) {
        color = vec4(67.0 / 255.0, 42.0 / 255.0, 42.0 / 255.0, 1.0);

    }
}

void main() {
    vec4 color = Color;

    color.a = phasing;

    # ifdef COLORMATERIAL_TEXTURE
        color *= texture(ColorMaterial_texture, v_Uv);
    # endif
    //set_color_of_player(color);
    //add_lighting(color);

    o_Target = color;
}
