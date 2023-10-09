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