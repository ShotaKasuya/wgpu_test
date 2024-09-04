use cgmath::{Matrix4, Quaternion, Vector3};

pub struct Instance {
    position: Vector3<f32>,
    rotation: Quaternion<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl Into<InstanceRaw> for Instance {
    fn into(self) -> InstanceRaw {
        InstanceRaw {
            model: (
                Matrix4::from_translation(self.position) * Matrix4::from(self.rotation)
            ).into()
        }
    }
}