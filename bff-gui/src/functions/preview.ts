import { invoke } from "@tauri-apps/api";
import { tempdir } from "@tauri-apps/api/os";
import { BigFileData, PreviewObject, PreviewTab } from "../types/types";

export async function updatePreview(
  objectName: number,
  setPreviewObject: React.Dispatch<React.SetStateAction<PreviewObject | null>>,
  setOpenPreviewTab: React.Dispatch<React.SetStateAction<PreviewTab>>
) {
  let tmp = await tempdir();
  invoke("parse_object", {
    objectName: objectName,
    tempPath: tmp,
  }).then((object) => {
    let previewObject = object as PreviewObject;
    setPreviewObject(previewObject);
    if (previewObject.error !== null) setOpenPreviewTab(PreviewTab.Error);
    else if (previewObject.preview_path !== null)
      setOpenPreviewTab(PreviewTab.Preview);
    else setOpenPreviewTab(PreviewTab.Data);
  });
}
