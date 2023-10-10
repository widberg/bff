import { invoke } from "@tauri-apps/api";
import {
  MeshBasicMaterial,
  MeshNormalMaterial,
  MeshStandardMaterial,
} from "three";

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

export const DEFAULT_MAT = new MeshStandardMaterial();
export const NORMAL_MAT = new MeshNormalMaterial();
export const WIREFRAME_MAT = new MeshBasicMaterial({ wireframe: true });

export const BIGFILE_EXTENSIONS = await invoke("get_extensions");
