#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform ColorMaterial_color {
    vec4 Color;
};

layout(set = 2, binding = 2) uniform HelmetColor_value {
    vec3 helmet_color;
};

layout(set = 2, binding = 3) uniform InnerSuitColor_value {
    vec3 inner_suit_color;
};

layout(set = 2, binding = 4) uniform WindowSize_value {
    vec2 screen_dimensions;
};

layout(set = 2, binding = 5) uniform Alpha_value {
    float phasing;
};

layout(set = 2, binding = 6) uniform AmbientLightLevel_value {
    float ambient_light_level;
};

/*layout(set = 2, binding = 7) uniform NumLights_value {
    int num_lights;
};*/

layout(set = 2, binding = 8) uniform Lights_value {
    uniform vec2 light_pos[32];
};



# ifdef COLORMATERIAL_TEXTURE 
layout(set = 1, binding = 1) uniform texture2D ColorMaterial_texture;
layout(set = 1, binding = 2) uniform sampler ColorMaterial_texture_sampler;
# endif

//Lighting settings
const float light_radius = 300.0;
const float max_light_intensity = 0.08;

// Light math
void add_lighting(inout vec4 color) {
    for (int i = 0; i < 32; i++) {
        vec2 light_pos = light_pos[i];
        if (light_pos != vec2(0.0, 0.0)) {
            vec2 pixel_pos = gl_FragCoord.xy;

            float light_distance = distance(light_pos, pixel_pos / screen_dimensions);
            float color_change = smoothstep(light_radius, 0.0, light_distance);

            color.rgb *= clamp(color_change, 0.0, max_light_intensity) + ambient_light_level;

        }

    }

}

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
    add_lighting(color);

    o_Target = color;
}
