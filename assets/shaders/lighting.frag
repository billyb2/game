#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform ColorMaterial_color {
    vec4 Color;
};

layout(set = 2, binding = 2) uniform WindowSize_value {
    vec2 screen_dimensions;
};

layout(set = 2, binding = 3) uniform NumLights_value {
    int num_lights;
};

layout(set = 2, binding = 4) uniform AmbientLightLevel_value {
    float ambient_light_level;
};

layout(set = 2, binding = 5) uniform Lights_value {
    uniform vec2 light_pos[32];
};


# ifdef COLORMATERIAL_TEXTURE 
layout(set = 1, binding = 1) uniform texture2D ColorMaterial_texture;
layout(set = 1, binding = 2) uniform sampler ColorMaterial_texture_sampler;
# endif

//Lighting settings
const float light_radius = 0.3;
const float max_light_intensity = 0.8;

// Light math
void add_lighting(inout vec4 color) {
    for (int i = 0; i < num_lights; i++) {
        vec2 light_pos = light_pos[i];
        vec2 pixel_pos = gl_FragCoord.xy;

        float light_distance = distance(light_pos, pixel_pos / screen_dimensions);
        float color_change = smoothstep(light_radius, 0.0, light_distance);

        color.rgb *= clamp(color_change, 0.0, max_light_intensity) + ambient_light_level;

    }

}

void main() {
    vec4 color = Color;

    # ifdef COLORMATERIAL_TEXTURE
        color *= texture(sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler), v_Uv);
    # endif

    add_lighting(color);

    o_Target = color;
}
