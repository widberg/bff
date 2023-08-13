//this is copied from dpc LOL
//TODO: get the name from the backend
export const CLASS_NAMES: Map<number, string> = new Map([
  [549480509, "Omni_Z"],
  [705810152, "Rtc_Z"],
  [838505646, "GenWorld_Z"],
  [848525546, "LightData_Z"],
  [849267944, "Sound_Z"],
  [849861735, "MaterialObj_Z"],
  [866453734, "RotShape_Z"],
  [954499543, "ParticlesData_Z"],
  [968261323, "World_Z"],
  [1114947943, "Warp_Z"],
  [1135194223, "Spline_Z"],
  [1175485833, "Animation_Z"],
  [1387343541, "Mesh_Z"],
  [1391959958, "UserDefine_Z"],
  [1396791303, "Skin_Z"],
  [1471281566, "Bitmap_Z"],
  [1536002910, "Fonts_Z"],
  [1625945536, "RotShapeData_Z"],
  [1706265229, "Surface_Z"],
  [1910554652, "SplineGraph_Z"],
  [1943824915, "Lod_Z"],
  [2204276779, "Material_Z"],
  [2245010728, "Node_Z"],
  [2259852416, "Binary_Z"],
  [2398393906, "CollisionVol_Z"],
  [2906362741, "WorldRef_Z"],
  [3312018398, "Particles_Z"],
  [3412401859, "LodData_Z"],
  [3611002348, "Skel_Z"],
  [3626109572, "MeshData_Z"],
  [3747817665, "SurfaceDatas_Z"],
  [3834418854, "MaterialAnim_Z"],
  [3845834591, "GwRoad_Z"],
  [4096629181, "GameObj_Z"],
  [4240844041, "Camera_Z"],
  [4117606081, "AnimFrame_Z"],
  [3979333606, "CameraZone_Z"],
  [72309972, "Occluder_Z"],
  [1390918523, "Graph_Z"],
  [1918499807, "Light_Z"],
  [3210467954, "HFogData_Z"],
  [2735949084, "HFog_Z"],
  [2203168663, "Flare_Z"],
  [1393846573, "FlareData_Z"],
]);

export const IMAGE_EXT = "png";
export const MESH_EXT = "dae";
export const SOUND_EXT = "wav";
export const JSON_EXT = "json";

export const EXTENSION_DESCRIPTIONS: Map<string, string> = new Map([
  [IMAGE_EXT, "Image"],
  [MESH_EXT, "Mesh"],
  [SOUND_EXT, "Sound"],
  [JSON_EXT, "JSON"],
]);

export const BIGFILE_EXTENSIONS = [
  "DPC",
  "DUA",
  "DMC",
  "DBM",
  "DPS",
  "DP3",
  "DPP",
  "DXB",
  "D36",
  "DGC",
  "DRV",
  "DNX",
]; //potentially get extensions from bff itself
