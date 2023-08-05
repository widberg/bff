import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { tempdir } from "@tauri-apps/api/os";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { JSX } from "react/jsx-runtime";

//this is copied from dpc LOL
//TODO: get the name from the backend
const classNames: Map<number, String> = new Map([
  [549480509, "Omni_Z"],
  [705810152, "Rtc_Z"],
  [838505646, "GenWorld_Z"],
  [848525546, "LightData_Z"],
  [849267944, "Sound_Z"],
  [849861735, "MaterialObj_Z"],
  [866453734, "RotShape_Z"],
  [954499543, "ParticlesData_Z"],
  [968261323, "World_Z"],
  [1114947943, "Warp_Z"],
  [1135194223, "Spline_Z"],
  [1175485833, "Animation_Z"],
  [1387343541, "Mesh_Z"],
  [1391959958, "UserDefine_Z"],
  [1396791303, "Skin_Z"],
  [1471281566, "Bitmap_Z"],
  [1536002910, "Fonts_Z"],
  [1625945536, "RotShapeData_Z"],
  [1706265229, "Surface_Z"],
  [1910554652, "SplineGraph_Z"],
  [1943824915, "Lod_Z"],
  [2204276779, "Material_Z"],
  [2245010728, "Node_Z"],
  [2259852416, "Binary_Z"],
  [2398393906, "CollisionVol_Z"],
  [2906362741, "WorldRef_Z"],
  [3312018398, "Particles_Z"],
  [3412401859, "LodData_Z"],
  [3611002348, "Skel_Z"],
  [3626109572, "MeshData_Z"],
  [3747817665, "SurfaceDatas_Z"],
  [3834418854, "MaterialAnim_Z"],
  [3845834591, "GwRoad_Z"],
  [4096629181, "GameObj_Z"],
  [4240844041, "Camera_Z"],
  [4117606081, "AnimFrame_Z"],
  [3979333606, "CameraZone_Z"],
  [72309972, "Occluder_Z"],
  [1390918523, "Graph_Z"],
  [1918499807, "Light_Z"],
  [3210467954, "HFogData_Z"],
  [2735949084, "HFog_Z"],
  [2203168663, "Flare_Z"],
  [1393846573, "FlareData_Z"],
]);

interface BigFileData {
  name: string;
  objects: BFFObject[];
}

interface BFFObject {
  name: number;
  class_name: number;
  is_implemented: boolean;
}

interface PreviewData {
  name: number;
  preview_data: string;
  preview_path?: string;
}

function BFFObjectButton({
  bffObjectName = "",
  implemented = true,
  index = 0,
  onClick,
}: {
  bffObjectName: String;
  implemented: boolean;
  index: number;
  onClick: any;
}) {
  return (
    <button
      className={`bffobject ${implemented ? "" : "bffobject-unimpl"}`}
      onClick={() => {
        onClick(index);
      }}
    >
      {bffObjectName}
    </button>
  );
}

function BFFObjects({
  bffObjects,
  onClick,
}: {
  bffObjects: BFFObject[];
  onClick: any;
}) {
  let btns: JSX.Element[] = [];
  bffObjects.map((v: BFFObject, i: number) => {
    btns.push(
      <BFFObjectButton
        key={i}
        implemented={v.is_implemented}
        bffObjectName={String(v.name) + "." + classNames.get(v.class_name)}
        index={i}
        onClick={onClick}
      />
    );
  });
  return <div>{btns}</div>;
}

function Preview({ previewPath }: { previewPath: string }) {
  if (previewPath.endsWith("png"))
    return (
      <img
        className="preview-display preview-image"
        src={previewPath}
        alt="image"
      />
    );
  else if (previewPath.endsWith("wav"))
    return <audio className="preview-display" controls src={previewPath} />;
}

function App(this: any) {
  const [bigfile, setBigfile] = useState<BigFileData>({
    name: "",
    objects: [],
  });
  const [currentBFFObject, setCurrentBFFObject] = useState<PreviewData | null>(
    null
  );

  listen("tauri://file-drop", (event) => {
    openBF((event.payload as Array<String>)[0]);
  });

  async function setBFFObject(objectIndex: number) {
    let tmp = await tempdir();
    let object = (await invoke("parse_object", {
      objectName: bigfile.objects[objectIndex].name,
      tempPath: tmp,
    })) as PreviewData;
    setCurrentBFFObject(object);
  }

  async function selectAndOpenBF() {
    const selected = (await open({
      multiple: false,
      filters: [
        {
          name: "BigFile",
          extensions: [
            "DPC",
            "DUA",
            "DMC",
            "DBM",
            "DPS",
            "DP3",
            "DPP",
            "DXB",
            "D36",
            "DGC",
            "DRV",
            "DNX",
          ], //potentially get extensions from bff itself
        },
      ],
    })) as string | null;

    if (selected === null) {
      return;
    }
    openBF(selected);
  }

  async function openBF(path: String) {
    setCurrentBFFObject(null);
    invoke("extract_bigfile", {
      path: path,
    })
      .then((bfData) => {
        setBigfile(bfData as BigFileData);
      })
      .catch((e) => console.error(e));
  }

  return (
    <div className="container">
      <div className="menubar">
        <button type="submit" onClick={selectAndOpenBF}>
          open bigfile...
        </button>
      </div>
      <div className="main">
        <div className="explorer">
          <span className="explorer-header">{bigfile.name}</span>
          <div className="bffobject-list">
            <BFFObjects bffObjects={bigfile.objects} onClick={setBFFObject} />
          </div>
        </div>
        <div className="preview">
          <span className="preview-header">
            {currentBFFObject !== null ? currentBFFObject.name : "preview"}
          </span>
          {currentBFFObject !== null && (
            <>
              <div className="preview-inner">
                {currentBFFObject.preview_path !== null ? (
                  <Preview
                    previewPath={convertFileSrc(
                      currentBFFObject.preview_path as string
                    )}
                  />
                ) : (
                  <div className="preview-text">
                    <p>{currentBFFObject.preview_data}</p>
                  </div>
                )}
              </div>
            </>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
