#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform ColorMaterial_color {
    vec4 Color;
};

layout(set = 2, binding = 2) uniform WindowSize_value {
    vec2 screen_dimensions;
};

layout(set = 2, binding = 3) uniform Lights_value {
    uniform vec2 light_pos[32];
};


# ifdef COLORMATERIAL_TEXTURE 
layout(set = 1, binding = 1) uniform texture2D ColorMaterial_texture;
layout(set = 1, binding = 2) uniform sampler ColorMaterial_texture_sampler;
# endif

//Lighting settings
const float light_radius = 0.4;
const float max_light_intensity = 0.5;

// Light math
void add_lighting(inout vec4 color) {
    for (int i = 0; i < 32; i++) {
        vec2 light_pos = light_pos[i];
        //TODO: Replace if statement with num_lights
        if (light_pos != vec2(0.0, 0.0)) {
            vec2 pixel_pos = gl_FragCoord.xy;

            float light_distance = distance(light_pos / screen_dimensions, pixel_pos / screen_dimensions);
            float color_change = smoothstep(light_radius, 0.0, light_distance);

            color.rgb *= color_change;

        }

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
