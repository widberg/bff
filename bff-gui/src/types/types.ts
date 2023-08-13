import { Material } from "three";

export interface BigFileData {
  name: string;
  objects: BFFObject[];
}

export interface BFFObject {
  name: number;
  class_name: number;
  is_implemented: boolean;
}

export interface PreviewObject {
  name: number;
  preview_data?: string;
  preview_path?: string;
  error?: string;
}

export interface MeshMaterial {
  name: string;
  material: Material;
}

export enum Sort {
  Block = 0,
  Name,
  Extension,
}

export enum Submenu {
  None = -1,
  Export,
}

export enum PreviewTab {
  Data = 0,
  Preview,
  Error,
}
