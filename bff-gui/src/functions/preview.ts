import { invoke } from "@tauri-apps/api";
import { tempdir } from "@tauri-apps/api/os";
import { BigFileData, ResourcePreview, ViewTab } from "../types/types";
import {message} from "@tauri-apps/api/dialog";

export async function updateView(
  resourceName: number,
  setPreviewObject: React.Dispatch<React.SetStateAction<ResourcePreview | null>>,
  setOpenTab: React.Dispatch<React.SetStateAction<ViewTab>>
) {
  let temp_dir = await tempdir();
  invoke("parse_resource", {
    resourceName: resourceName,
    tempPath: temp_dir,
  }).then((preview) => {
    let resourcePreview = preview as ResourcePreview;
    setPreviewObject(resourcePreview);
    // if (resourcePreview.error !== null) setOpenTab(PreviewTab.Error);
    if (resourcePreview.preview_path !== null)
      setOpenTab(ViewTab.Preview);
    else setOpenTab(ViewTab.Data);
  }).catch((e) => message(e, { type: "warning" }));
}
