struct Vertex {
    @location(0) position: vec3<f32>,
};

struct TerrainMaterial {
    @location(0) heights: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec3<f32>,
};

@group(2) @binding(0)
var<uniform> material: TerrainMaterial;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var output: VertexOutput;

    output.clip_position = vertex.position;


    return output;
}
