import { invoke } from "@tauri-apps/api";
import { message, save, open } from "@tauri-apps/api/dialog";

import { EXTENSION_DESCRIPTIONS, JSON_EXT } from "../constants/constants";
import { extname } from "@tauri-apps/api/path";

export async function exportAll() {
  open({ directory: true }).then((path) => {
    if (path !== null)
      invoke("export_all_objects", { path: path }).catch((e) =>
        message(e, { type: "warning" })
      );
  });
}

export async function exportOne(objectName: number) {
  save({
    defaultPath: `${objectName}.${JSON_EXT}`,
    filters: [
      {
        name: EXTENSION_DESCRIPTIONS.get(JSON_EXT) as string,
        extensions: [JSON_EXT],
      },
    ],
  }).then((path) => {
    if (path !== null)
      invoke("export_one_object", { path: path, name: objectName }).catch((e) =>
        message(e, { type: "warning" })
      );
  });
}

export async function exportPreview(objectName: number, objectPath: string) {
  let extension = await extname(objectPath);
  save({
    defaultPath: `${objectName}.${extension}`,
    filters: [
      {
        name: EXTENSION_DESCRIPTIONS.get(extension) as string,
        extensions: [extension],
      },
    ],
  }).then((path) => {
    if (path !== null)
      invoke("export_preview", { path: path, name: objectName }).catch((e) =>
        message(e, { type: "warning" })
      );
  });
}
