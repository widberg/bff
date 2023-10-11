import { message, open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";
import { BigFileData, ResourcePreview } from "../types/types";

export async function selectBigfile(
  setPreviewObject: React.Dispatch<
    React.SetStateAction<ResourcePreview | null>
  >,
  setBigfile: React.Dispatch<React.SetStateAction<BigFileData>>
) {
  open({
    multiple: false,
    filters: [
      {
        name: "BigFile",
        extensions: await invoke("get_extensions"),
      },
    ],
  }).then((path) => {
    if (path !== null) {
      openBigfile(path as string, setPreviewObject, setBigfile);
    }
  });
}

export async function openBigfile(
  path: string,
  setPreviewObject: React.Dispatch<
    React.SetStateAction<ResourcePreview | null>
  >,
  setBigfile: React.Dispatch<React.SetStateAction<BigFileData>>
) {
  setPreviewObject(null);
  invoke("extract_bigfile", {
    path: path,
  })
    .then((bfData) => {
      setBigfile(bfData as BigFileData);
    })
    .catch((e) => message(e, { type: "warning" }));
}
