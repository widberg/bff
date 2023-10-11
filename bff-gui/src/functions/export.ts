import { invoke } from "@tauri-apps/api";
import { message, save, open } from "@tauri-apps/api/dialog";
import { basename } from "@tauri-apps/api/path";

import { EXTENSIONS } from "../constants/constants";
import { BigFileData, DataType, Nickname, ResourceInfo } from "../types/types";

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
    defaultPath: `${objectName}.${
      (EXTENSIONS.get(DataType.Json) as string[])[0]
    }`,
    filters: [
      {
        name: (EXTENSIONS.get(DataType.Json) as string[])[1],
        extensions: [(EXTENSIONS.get(DataType.Json) as string[])[0]],
      },
    ],
  }).then((path) => {
    if (path !== null)
      invoke("export_one_object", { path: path, name: objectName }).catch((e) =>
        message(e, { type: "warning" })
      );
  });
}

export async function exportPreview(resourceName: number, dataType: DataType) {
  let extInfo = EXTENSIONS.get(dataType) as string[];
  save({
    defaultPath: `${resourceName}.${extInfo[0]}`,
    filters: [
      {
        name: extInfo[1],
        extensions: [extInfo[0]],
      },
    ],
  }).then((path) => {
    if (path !== null)
      invoke("export_preview", { path: path, name: resourceName }).catch((e) =>
        message(e, { type: "warning" })
      );
  });
}

export async function exportNicknames(
  bigfileName: string,
  nicknames: Nickname[]
) {
  let extInfo = EXTENSIONS.get(DataType.Json) as string[];
  let bfBasename = await basename(bigfileName);
  save({
    defaultPath: `${bfBasename}-nicknames.${extInfo[0]}`,
    filters: [
      {
        name: extInfo[1],
        extensions: [extInfo[0]],
      },
    ],
  }).then((path) => {
    if (path !== null)
      invoke("export_nicknames", {
        path: path,
        nicknames: new Map(nicknames.map((v) => [v.name, v.nickname] as const)),
      }).catch((e) => message(e, { type: "warning" }));
  });
}

export async function importNicknames(
  setNicknames: React.Dispatch<React.SetStateAction<Nickname[]>>
) {
  let extInfo = EXTENSIONS.get(DataType.Json) as string[];
  open({
    multiple: false,
    filters: [
      {
        name: extInfo[1],
        extensions: [extInfo[0]],
      },
    ],
  }).then((path) => {
    if (path !== null)
      invoke("import_nicknames", {
        path: path,
      })
        .then((nicknames) => setNicknames(nicknames as Nickname[]))
        .catch((e) => message(e, { type: "warning" }));
  });
}
