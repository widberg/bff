use std::sync::Arc;

use itertools::MultiUnzip;
use three_d::CpuModel;

use super::export::Export;
use crate::artifact::Artifact;

pub trait GenerateMesh {
    fn generate_mesh(&self) -> Vec<three_d::geometry::CpuMesh>;
}

impl GenerateMesh for bff::class::mesh::v1_291_03_06_pc::MeshV1_291_03_06PC {
    fn generate_mesh(&self) -> Vec<three_d::geometry::CpuMesh> {
        use three_d::{Vec2, Vec3, Vec4};
        self.body
            .mesh_buffers
            .vertex_groups
            .iter()
            .map(|group| {
                // println!("{}", mesh.body.mesh_buffer.vertex_buffers.len());
                let (positions, uvs, normals, tangents): (
                    Vec<Vec3>,
                    Vec<Vec2>,
                    Vec<Vec3>,
                    Vec<Vec4>,
                ) = self
                    .body
                    .mesh_buffers
                    .vertex_buffers
                    .iter()
                    .flat_map(|buf| &buf.vertex_structs)
                    .collect::<Vec<&bff::class::mesh::v1_291_03_06_pc::VertexStruct>>()
                    [group.vertex_offset_in_groups as usize
                        ..group.vertex_offset_in_groups as usize + group.vertex_count as usize]
                    .iter()
                    .map(|vs| match vs {
                        bff::class::mesh::v1_291_03_06_pc::VertexStruct::Format24 {
                            position,
                            uv,
                            ..
                        } => (position, uv, &[0u8; 3], [0u8; 4]),
                        bff::class::mesh::v1_291_03_06_pc::VertexStruct::Format36 {
                            position,
                            uv,
                            normal,
                            tangent,
                            tangent_padding,
                            ..
                        }
                        | bff::class::mesh::v1_291_03_06_pc::VertexStruct::Format48 {
                            position,
                            uv,
                            normal,
                            tangent,
                            tangent_padding,
                            ..
                        }
                        | bff::class::mesh::v1_291_03_06_pc::VertexStruct::Format60 {
                            position,
                            uv,
                            normal,
                            tangent,
                            tangent_padding,
                            ..
                        } => (
                            position,
                            uv,
                            normal,
                            [&tangent[..], &[*tangent_padding]]
                                .concat()
                                .try_into()
                                .unwrap(),
                        ),
                        bff::class::mesh::v1_291_03_06_pc::VertexStruct::FormatUnknown {
                            ..
                        } => (&[0f32; 3], &[0f32; 2], &[0u8; 3], [0u8; 4]),
                    })
                    .map(|(p, u, n, t)| {
                        (
                            Vec3::from(*p),
                            Vec2::from(*u),
                            {
                                let mut norm = n.map(|i| (i as f32 - 128.0) / 128.0);
                                norm[2] *= -1.0;
                                Vec3::from(norm)
                            },
                            Vec4::from(t.map(|i| (i as f32 - 128.0) / 128.0)),
                        )
                    })
                    .multiunzip();
                let indices: Vec<u16> = self
                    .body
                    .mesh_buffers
                    .index_buffers
                    .iter()
                    .flat_map(|buf| &buf.tris)
                    .flat_map(|tri| tri.indices)
                    .collect::<Vec<i16>>()[group.index_buffer_offset_in_shorts
                    as usize
                    ..group.index_buffer_offset_in_shorts as usize + group.face_count as usize * 3]
                    .iter()
                    .map(|i| u16::try_from(*i).unwrap_or(0) - group.vertex_offset_in_groups)
                    .collect();
                three_d::geometry::CpuMesh {
                    positions: three_d::Positions::F32(positions),
                    indices: three_d::Indices::U16(indices),
                    normals: Some(normals),
                    tangents: Some(tangents),
                    uvs: Some(uvs),
                    colors: None,
                }
            })
            .collect()
    }
}

impl Export for bff::class::mesh::v1_291_03_06_pc::MeshV1_291_03_06PC {
    fn export(self) -> Artifact {
        let tri_meshes = self.generate_mesh();
        let primitives = tri_meshes
            .into_iter()
            .map(|m| {
                let triangles = three_d_asset::geometry::Geometry::Triangles(m);
                three_d_asset::Primitive {
                    name: "mesh".to_string(),
                    transformation: three_d_asset::Mat4::from_translation([0.0; 3].into()),
                    animations: vec![],
                    geometry: triangles,
                    material_index: None,
                }
            })
            .collect();
        let model = CpuModel {
            name: self.name.to_string(),
            geometries: primitives,
            materials: vec![],
        };
        Artifact::Mesh(Arc::new(model))
    }
}
