#version 300 es
precision highp float;

in vec2 v_Uv;
out vec4 o_Target;

layout(std140) uniform ColorMaterial_color {
    vec4 Color;
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

layout(std140) uniform AmbientLightLevel_value {
    float ambient_light_level;
};

layout(std140) uniform Alpha_value {
    float phasing;
};

layout(std140) uniform Lights_value {
    vec2 light_pos[8];
};


# ifdef COLORMATERIAL_TEXTURE 
uniform sampler2D ColorMaterial_texture;  // set = 1, binding = 1
# endif


//Lighting settings
const float light_radius = 300.0;
const float max_light_intensity = 0.12;

// Light math
void add_lighting(inout vec4 color) {
    for (int i = 0; i < 8; i++) {
        vec2 light_pos = light_pos[i];
        if (light_pos != vec2(0.0, 0.0)) {
            vec2 pixel_pos = gl_FragCoord.xy;

            float light_distance = distance(light_pos, pixel_pos / screen_dimensions);
            float color_change = smoothstep(light_radius, 0.0, light_distance);

            color.rgb *= clamp(color_change, 0.0, max_light_intensity) + ambient_light_level;

        }

    }
}

vec4 color_encode(vec4 linearRGB_in) {
    vec3 linearRGB = linearRGB_in.rgb;
    vec3 a = 12.92 * linearRGB;
    vec3 b = 1.055 * pow(linearRGB, vec3(1.0 / 2.4)) - 0.055;
    vec3 c = step(vec3(0.0031308), linearRGB);
    return vec4(mix(a, b, c), linearRGB_in.a);
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

    # ifdef COLORMATERIAL_TEXTURE
        color *= texture(ColorMaterial_texture, v_Uv);
    # endif
    set_color_of_player(color);
    add_lighting(color);

    o_Target = color_encode(color);
}