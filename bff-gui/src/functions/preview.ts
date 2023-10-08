import { invoke } from "@tauri-apps/api";
import { tempdir } from "@tauri-apps/api/os";
import { BigFileData, ResourcePreview, PreviewTab } from "../types/types";

export async function updatePreview(
  resourceName: number,
  setPreviewObject: React.Dispatch<React.SetStateAction<ResourcePreview | null>>,
  setOpenPreviewTab: React.Dispatch<React.SetStateAction<PreviewTab>>
) {
  let temp_dir = await tempdir();
  invoke("parse_object", {
    resourceName: resourceName,
    tempPath: temp_dir,
  }).then((preview) => {
    let resourcePreview = preview as ResourcePreview;
    setPreviewObject(resourcePreview);
    // if (resourcePreview.error !== null) setOpenPreviewTab(PreviewTab.Error);
    if (resourcePreview.preview_path !== null)
      setOpenPreviewTab(PreviewTab.Preview);
    else setOpenPreviewTab(PreviewTab.Data);
  }).catch((e) => console.error(e));
}
