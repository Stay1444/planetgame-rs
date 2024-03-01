struct TerrainMaterial {
    @location(0) color: vec4<f32>,
};

@group(2) @binding(0)
var<uniform> material: TerrainMaterial;

// Function to map a height value to a color
fn colorFromHeight(height: f32) -> vec4<f32> {
    // Example gradient mapping, you can customize this
    if height < 0.3 {
        return vec4<f32>(0.0, 0.5, 1.0, 1.0); // Blue for low heights (water)
    } else if height < 0.6 {
        return vec4<f32>(0.8, 0.8, 0.8, 1.0); // Gray for mid-heights (rocky)
    } else {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0); // White for high heights (snowy)
    }
}

@fragment
fn fragment() -> @location(0) vec4<f32> {
    // You need to pass the height value from your vertex shader
    // For example, you could pass it as another attribute like this:
    // var<in> height: f32;

    // Then calculate the color based on the height value
    let color = colorFromHeight(material.color.r); // Assuming red channel of color represents height

    return vec4<f32>(color);
}
