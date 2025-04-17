use std::collections::HashMap;
use std::sync::Arc;

use bff::class::Class;
use bff::names::Name;

use super::export::RecursiveExport;
use super::mesh::GenerateMesh;
use crate::artifact::Artifact;

impl RecursiveExport for bff::class::skin::v1_291_03_06_pc::SkinV1_291_03_06PC {
    fn dependencies(&self) -> Vec<Name> {
        let material_crc32s: Vec<Name> = self
            .body
            .skin_sections
            .iter()
            .flat_map(|section| &section.skin_sub_sections.inner)
            .map(|subsection| subsection.material_name)
            .collect();
        [self.body.mesh_names.inner.clone(), material_crc32s].concat()
    }
    fn export(self, classes: &HashMap<Name, Class>) -> Artifact {
        let tri_meshes: Vec<three_d_asset::Primitive> = self
            .body
            .mesh_names
            .iter()
            .flat_map(|n| {
                let class = classes.get(n).unwrap();
                match class {
                    Class::Mesh(mesh) => match *mesh {
                        bff::class::mesh::Mesh::MeshV1_291_03_06PC(ref mesh) => {
                            mesh.generate_mesh()
                        }
                        _ => todo!(),
                    },
                    _ => panic!("not a mesh?"),
                }
            })
            .enumerate()
            .map(|(i, mesh)| {
                let triangles = three_d_asset::geometry::Geometry::Triangles(mesh);
                three_d_asset::Primitive {
                    name: format!("skin-part{}", i),
                    transformation: three_d_asset::Mat4::from_translation([0.0; 3].into()),
                    animations: vec![],
                    geometry: triangles,
                    material_index: Some(i),
                }
            })
            .collect();
        let materials = self
            .body
            .skin_sections
            .iter()
            .flat_map(|section| &section.skin_sub_sections.inner)
            .enumerate()
            .map(|(i, subsection)| {
                if let Some(class) = classes.get(&subsection.material_name) {
                    match class {
                        Class::Material(material) => match *material {
                            bff::class::material::Material::MaterialV1_291_03_06PC(
                                ref material,
                            ) => three_d::renderer::material::CpuMaterial {
                                name: format!("{}-mat{}", subsection.material_name, i),
                                albedo: material.body.diffuse.into(),
                                emissive: material.body.emission.into(),
                                ..Default::default()
                            },
                            _ => todo!(),
                        },
                        _ => panic!("not a material?"),
                    }
                } else {
                    three_d::renderer::material::CpuMaterial {
                        name: format!("{}-mat", subsection.material_name),
                        ..Default::default()
                    }
                }
            })
            .collect();

        let model = three_d::renderer::object::CpuModel {
            name: self.name.to_string(),
            geometries: tri_meshes,
            materials,
        };
        Artifact::Skin(Arc::new(model))
    }
}
