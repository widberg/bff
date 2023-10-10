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
  preview_json: string;
  preview_data?: PreviewData;
}

export interface Nickname {
  name: number;
  nickname: string;
}

export interface PreviewData {
  is_base64: boolean;
  data: string;
  data_type: DataType;
}

export enum DataType {
  Image,
  Sound,
  Mesh,
  Text,
  Json,
}

export interface MeshMaterial {
  name: string;
  material: Material;
}

export enum MaterialType {
  Default = "default",
  Normal = "normal",
  Wireframe = "wireframe",
}

export enum Sort {
  Default,
  Name,
  Extension,
}

export enum Submenu {
  None = -1,
  Export,
}

export enum ViewTab {
  Data,
  Preview,
}
