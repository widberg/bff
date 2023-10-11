import { basename } from "@tauri-apps/api/path";
import { EXTENSIONS } from "../constants/constants";
import { DataType, Nickname } from "../types/types";
import { message, save, open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";

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
        path: path as string,
      })
        .then((nicknames) => setNicknames(nicknames as Nickname[]))
        .catch((e) => message(e, { type: "warning" }));
  });
}

export async function clearNicknames(
  setNicknames: React.Dispatch<React.SetStateAction<Nickname[]>>
) {
  setNicknames([]);
}
