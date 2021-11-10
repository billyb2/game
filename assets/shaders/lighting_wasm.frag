#version 300 es
precision highp float;

in vec2 v_Uv;
out vec4 o_Target;

layout(std140) uniform ColorMaterial_color {
    vec4 Color;
};

layout(std140) uniform WindowSize_value {
    vec2 screen_dimensions;
};

layout(std140) uniform Lights_value {
    uniform vec2 light_pos[32];
};


# ifdef COLORMATERIAL_TEXTURE 
uniform sampler2D ColorMaterial_texture;
# endif

vec4 color_encode(vec4 linearRGB_in) {
    vec3 linearRGB = linearRGB_in.rgb;
    vec3 a = 12.92 * linearRGB;
    vec3 b = 1.055 * pow(linearRGB, vec3(1.0 / 2.4)) - 0.055;
    vec3 c = step(vec3(0.0031308), linearRGB);
    return vec4(mix(a, b, c), linearRGB_in.a);
}

//Lighting settings
const float light_radius = 0.4;
const float max_light_intensity = 0.5;

// Light math
void add_lighting(inout vec4 color) {
    for (int i = 0; i < 32; i++) {
        vec2 light_pos = light_pos[i];
        //TODO: Replace if statement with num_lights, since branching is bad
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
        color *= texture(ColorMaterial_texture, v_Uv);
    # endif

    add_lighting(color);

    o_Target = color;
}
