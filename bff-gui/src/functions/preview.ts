import { invoke } from "@tauri-apps/api";
import { ResourcePreview, ViewTab } from "../types/types";
import { message } from "@tauri-apps/api/dialog";

export async function updateView(
  resourceName: number,
  setPreviewObject: React.Dispatch<
    React.SetStateAction<ResourcePreview | null>
  >,
  setOpenTab: React.Dispatch<React.SetStateAction<ViewTab>>
) {
  invoke("parse_resource", {
    resourceName: resourceName,
  })
    .then((preview) => {
      let resourcePreview = preview as ResourcePreview;
      setPreviewObject(resourcePreview);
      if (resourcePreview.preview_data !== null) setOpenTab(ViewTab.Preview);
      else setOpenTab(ViewTab.Data);
    })
    .catch((e) => message(e, { type: "warning" }));
}
