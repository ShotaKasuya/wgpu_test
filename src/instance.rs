use cgmath::{Matrix4, Quaternion, Vector3};
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

pub const NUM_INSTANCES_PER_ROW: u32 = 10;
pub const INSTANCE_DISPLACEMENT: Vector3<f32> = Vector3::new(
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
    0.0,
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
);

pub struct Instance {
    pub(crate) position: Vector3<f32>,
    pub(crate) rotation: Quaternion<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (Matrix4::from_translation(self.position) * Matrix4::from(self.rotation)).into(),
        }
    }
}

impl InstanceRaw {
    pub fn desc() -> VertexBufferLayout<'static> {
        use std::mem;
        VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as BufferAddress,
            // 頂点のステップモードを`Instance`にする必要があります。
            // これは、シェーダーが新しいインスタンスの処理を開始したときにのみ、
            // シェーダが次のインスタンスを使用するように変更されることを意味します。
            step_mode: VertexStepMode::Instance,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: 6,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as BufferAddress,
                    shader_location: 7,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as BufferAddress,
                    shader_location: 8,
                    format: VertexFormat::Float32x4,
                },
            ],
        }
    }
}
