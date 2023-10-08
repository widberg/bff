import { Material } from "three";

export interface BigFileData {
  filename: string;
  resource_infos: ResourceInfo[];
}

export interface ResourceInfo {
  name: number;
  class_name: string;
}

export interface ResourcePreview {
  name: number;
  preview_data: string;
  preview_path?: string;
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
