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
                type VertexInfo = ([f32; 3], [f32; 2], [u8; 3], [u8; 4]);
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
                    .flat_map(|buf| match &buf.vertices {
                        bff::class::mesh::generic::Vertices::LayoutPosition(layout_positions) => {
                            Box::new(
                                layout_positions
                                    .iter()
                                    .map(|vs| (vs.position, [0f32; 2], [0u8; 3], [0u8; 4])),
                            ) as Box<dyn Iterator<Item = VertexInfo>>
                        }
                        bff::class::mesh::generic::Vertices::LayoutPositionUV(
                            layout_position_uvs,
                        ) => Box::new(
                            layout_position_uvs
                                .iter()
                                .map(|vs| (vs.position, vs.uv, [0u8; 3], [0u8; 4])),
                        ) as Box<dyn Iterator<Item = VertexInfo>>,
                        bff::class::mesh::generic::Vertices::LayoutNoBlend(layout_no_blends) => {
                            Box::new(layout_no_blends.iter().map(|vs| {
                                (
                                    vs.position,
                                    vs.uv,
                                    vs.normal,
                                    [&vs.tangent[..], &[vs.tangent_w]]
                                        .concat()
                                        .try_into()
                                        .unwrap(),
                                )
                            })) as Box<dyn Iterator<Item = VertexInfo>>
                        }
                        bff::class::mesh::generic::Vertices::Layout1Blend(layout1_blends) => {
                            Box::new(layout1_blends.iter().map(|vs| {
                                (
                                    vs.position,
                                    vs.uv,
                                    vs.normal,
                                    [&vs.tangent[..], &[vs.tangent_w]]
                                        .concat()
                                        .try_into()
                                        .unwrap(),
                                )
                            })) as Box<dyn Iterator<Item = VertexInfo>>
                        }
                        bff::class::mesh::generic::Vertices::Layout4Blend(layout4_blends) => {
                            Box::new(layout4_blends.iter().map(|vs| {
                                (
                                    vs.position,
                                    vs.uv,
                                    vs.normal,
                                    [&vs.tangent[..], &[vs.tangent_w]]
                                        .concat()
                                        .try_into()
                                        .unwrap(),
                                )
                            })) as Box<dyn Iterator<Item = VertexInfo>>
                        }
                        bff::class::mesh::generic::Vertices::LayoutUnknown { data, .. } => {
                            Box::new(
                                data.iter()
                                    .map(|_| ([0f32; 3], [0f32; 2], [0u8; 3], [0u8; 4])),
                            ) as Box<dyn Iterator<Item = VertexInfo>>
                        }
                    })
                    .skip(group.vertex_offset_in_groups as usize)
                    .take(
                        (group.vertex_offset_in_groups as usize + group.vertex_count as usize)
                            - group.vertex_offset_in_groups as usize,
                    )
                    .map(|(p, u, n, t)| {
                        (
                            Vec3::from(p),
                            Vec2::from(u),
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
                    .flatten()
                    .collect::<Vec<&i16>>()[group.index_buffer_index_begin as usize
                    ..group.index_buffer_index_begin as usize + group.face_count as usize * 3]
                    .iter()
                    .map(|i| u16::try_from(**i).unwrap_or(0) - group.vertex_offset_in_groups)
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
                    name: "mesh".to_owned(),
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
