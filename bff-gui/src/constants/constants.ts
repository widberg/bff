import {
  MeshBasicMaterial,
  MeshNormalMaterial,
  MeshStandardMaterial,
} from "three";
import { DataType } from "../types/types";

export const EXTENSIONS: Map<DataType, string[]> = new Map([
  [DataType.Image, ["png", "Image"]],
  [DataType.Sound, ["wav", "Sound"]],
  [DataType.Mesh, ["dae", "Mesh"]],
  [DataType.Text, ["txt", "Text"]],
  [DataType.Json, ["json", "JSON"]],
]);

export const DEFAULT_MAT = new MeshStandardMaterial();
export const NORMAL_MAT = new MeshNormalMaterial();
export const WIREFRAME_MAT = new MeshBasicMaterial({ wireframe: true });
