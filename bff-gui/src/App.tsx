import { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open, message, save } from "@tauri-apps/api/dialog";
import { tempdir } from "@tauri-apps/api/os";
import { extname } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { JSX } from "react/jsx-runtime";
import { ColladaLoader } from "three/examples/jsm/loaders/ColladaLoader";
import { Canvas, useLoader } from "@react-three/fiber";
import { OrbitControls } from "@react-three/drei";
import {
  DoubleSide,
  FrontSide,
  Material,
  MeshBasicMaterial,
  MeshNormalMaterial,
  MeshStandardMaterial,
} from "three";
import { TransformWrapper, TransformComponent } from "react-zoom-pan-pinch";
import parse from "html-react-parser";

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

interface PreviewObject {
  name: number;
  preview_data?: string;
  preview_path?: string;
  error?: string;
}

interface MeshMaterial {
  name: string;
  material: Material;
}

interface ParseError {
  error: string;
  object: PreviewObject;
}

function BFFObjectButton({
  bffObjectName = "",
  implemented = true,
  name = 0,
  onClick,
}: {
  bffObjectName: string;
  implemented: boolean;
  name: number;
  onClick: any;
}) {
  return (
    <button
      className={`bffobject ${implemented ? "" : "bffobject-unimpl"}`}
      onClick={() => {
        onClick(name);
      }}
    >
      {bffObjectName}
    </button>
  );
}

function BFFObjects({
  bffObjects,
  onClick,
  sort,
  sortBackward,
}: {
  bffObjects: BFFObject[];
  onClick: any;
  sort: number;
  sortBackward: boolean;
}) {
  let objectsCopy = [...bffObjects];
  if (sort == 1) objectsCopy.sort((a, b) => a.name - b.name);
  else if (sort == 2)
    objectsCopy.sort((a, b) =>
      (classNames.get(a.class_name) as string).localeCompare(
        classNames.get(b.class_name) as string
      )
    );
  if (sortBackward) objectsCopy.reverse();

  let btns: JSX.Element[] = objectsCopy.map((v: BFFObject, i: number) => (
    <BFFObjectButton
      key={i}
      implemented={v.is_implemented}
      bffObjectName={String(v.name) + "." + classNames.get(v.class_name)}
      name={v.name}
      onClick={onClick}
    />
  ));
  return <div>{btns}</div>;
}

function PreviewContainer({
  openTab,
  previewObject,
}: {
  openTab: number;
  previewObject: PreviewObject;
}) {
  if (openTab == 0)
    return (
      <div className="preview-data preview-text">
        <p>{previewObject.preview_data}</p>
      </div>
    );
  if (openTab == 1) {
    if (previewObject.preview_path !== null)
      return (
        <Preview
          previewPath={convertFileSrc(previewObject.preview_path as string)}
        />
      );
    else return <p className="preview-text">Preview unavailable</p>;
  }
  if (openTab == 2) {
    if (previewObject.error)
      return <p className="preview-text">{parse(previewObject.error)}</p>;
    else return <p className="preview-text">Loaded successfully</p>;
  }
}

function Preview({ previewPath }: { previewPath: string }) {
  const [rendering, setRendering] = useState<string>("pixelated");
  const [material, setMaterial] = useState<MeshMaterial>({
    name: "default",
    material: new MeshStandardMaterial(),
  });

  function setFilter(enabled: boolean) {
    setRendering(enabled ? "auto" : "pixelated");
  }
  function changeMaterial(type: string) {
    let mat: Material = new MeshStandardMaterial();
    if (type == "normal") mat = new MeshNormalMaterial();
    else if (type == "wireframe")
      mat = new MeshBasicMaterial({ wireframe: true });
    mat.side = material.material.side;
    setMaterial({
      name: type,
      material: mat,
    });
  }
  function setSide(checked: boolean) {
    material.material.side = checked ? DoubleSide : FrontSide;
  }

  if (previewPath.endsWith("png")) {
    return (
      <TransformWrapper minScale={0.1} limitToBounds={false}>
        <div className="preview-overlay">
          <div>
            <label htmlFor="filter">Filter</label>
            <input
              type="checkbox"
              id="filter"
              defaultChecked={rendering == "auto"}
              onChange={(e) => setFilter(e.target.checked)}
            />
          </div>
        </div>
        <TransformComponent>
          <img
            //@ts-ignore
            style={{ imageRendering: rendering }}
            src={previewPath}
            alt="image"
            className="preview-image"
          />
        </TransformComponent>
      </TransformWrapper>
    );
  } else if (previewPath.endsWith("wav"))
    return (
      <div className="preview-display">
        <audio controls src={previewPath} />
      </div>
    );
  else if (previewPath.endsWith("dae")) {
    const { scene } = useLoader(ColladaLoader, previewPath);

    return (
      <div className="preview-scene">
        <div className="preview-overlay">
          <label htmlFor="material">Material</label>
          <select
            name="material"
            id="material"
            defaultValue={material.name}
            onChange={(e) => changeMaterial(e.target.value)}
          >
            <option value="default">Default</option>
            <option value="normal">Normal</option>
            <option value="wireframe">Wireframe</option>
          </select>
          <div>
            <label htmlFor="sided">Double sided</label>
            <input
              type="checkbox"
              id="sided"
              defaultChecked={material.material.side == DoubleSide}
              onChange={(e) => setSide(e.target.checked)}
            />
          </div>
        </div>
        <Canvas
          camera={{ fov: 70, position: [0, 0, 5] }}
          resize={{ scroll: false, debounce: { scroll: 0, resize: 0 } }}
        >
          <OrbitControls rotateSpeed={0.7} dampingFactor={0.1} makeDefault />
          <ambientLight intensity={0.1} />
          <directionalLight color="white" position={[10, 10, 10]} />
          <directionalLight color="white" position={[-10, -10, -10]} />
          <group>
            <primitive object={scene} children-0-material={material.material} />
          </group>
        </Canvas>
      </div>
    );
  } else {
    return <></>;
  }
}

function SortButton({
  onClick,
  id,
  name,
  sort,
  sortBackward,
}: {
  onClick: any;
  id: number;
  name: string;
  sort: number;
  sortBackward: boolean;
}) {
  return (
    <button onClick={() => onClick(id)}>
      <span>{name}</span>
      {sort == id && (
        <span className="explorer-sort-arrow">{sortBackward ? "▲" : "▼"}</span>
      )}
    </button>
  );
}

function App(this: any) {
  const [bigfile, setBigfile] = useState<BigFileData>({
    name: "",
    objects: [],
  });
  const [currentBFFObject, setCurrentBFFObject] =
    useState<PreviewObject | null>(null);
  const [sortBackward, setSortBackward] = useState<boolean>(false);
  const [sort, setSort] = useState<number>(0);
  const [submenuShown, setSubmenuShown] = useState<number>(-1);
  const submenuRef: React.MutableRefObject<HTMLDivElement | null> =
    useRef(null);
  const [openPreviewTab, setOpenPreviewTab] = useState<number>(1);

  async function setBFFObject(objectName: number) {
    let tmp = await tempdir();
    invoke("parse_object", {
      objectName: objectName,
      tempPath: tmp,
    }).then((object) => {
      let previewObject = object as PreviewObject;
      setCurrentBFFObject(previewObject);
      if (previewObject.error !== null) setOpenPreviewTab(2);
      else if (previewObject.preview_path !== null) setOpenPreviewTab(1);
      else setOpenPreviewTab(0);
    });
  }

  async function selectAndOpenBF() {
    open({
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
    }).then((path) => {
      if (path !== null) openBF(path as string);
    });
  }

  async function openBF(path: String) {
    setCurrentBFFObject(null);
    invoke("extract_bigfile", {
      path: path,
    })
      .then((bfData) => {
        setBigfile(bfData as BigFileData);
      })
      .catch((e) => message(e, { type: "warning" }));
  }

  async function exportAll() {
    open({ directory: true }).then((path) => {
      if (path !== null)
        invoke("export_all_objects", { path: path }).catch((e) =>
          console.log(e)
        );
    });
  }

  async function exportOne(objectName: number) {
    save({
      defaultPath: `${objectName}.json`,
      filters: [
        {
          name: "JSON",
          extensions: ["json"],
        },
      ],
    }).then((path) => {
      if (path !== null)
        invoke("export_one_object", { path: path, name: objectName }).catch(
          (e) => console.log(e)
        );
    });
  }

  async function exportPreview(objectName: number, objectPath: string) {
    let extension = await extname(objectPath);
    save({
      defaultPath: `${objectName}.${extension}`,
      filters: [
        {
          name: extension,
          extensions: [extension],
        },
      ],
    }).then((path) => {
      if (path !== null)
        invoke("export_preview", { path: path, name: objectName }).catch((e) =>
          console.log(e)
        );
    });
  }

  function sortButtonPress(type: number) {
    setSort(type);
    setSortBackward(sort != type ? false : !sortBackward);
  }

  const handleClickOutside = (e: { target: any }) => {
    if (submenuRef.current && !submenuRef.current.contains(e.target)) {
      setSubmenuShown(-1);
    }
  };

  useEffect(() => {
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  });

  return (
    <div className="container">
      <div className="menubar">
        <button onClick={selectAndOpenBF}>Open BigFile...</button>
        <div ref={submenuRef}>
          <button
            onClick={() => {
              setSubmenuShown(submenuShown == 0 ? -1 : 0);
            }}
          >
            Export
          </button>
          <div
            className="submenu"
            style={{ display: submenuShown == 0 ? "flex" : "none" }}
          >
            <button onClick={exportAll} disabled={!bigfile.name}>
              Export objects as JSON...
            </button>
            <button
              onClick={() => exportOne(currentBFFObject?.name as number)}
              disabled={currentBFFObject === null}
            >
              Export current object as JSON...
            </button>
            <button
              onClick={() =>
                exportPreview(
                  currentBFFObject?.name as number,
                  currentBFFObject?.preview_path as string
                )
              }
              disabled={
                currentBFFObject === null ||
                currentBFFObject?.preview_path === null
              }
            >
              Export preview...
            </button>
          </div>
        </div>
      </div>
      <div className="main">
        <div className="explorer">
          <span className="explorer-header">
            {bigfile.name !== "" ? bigfile.name : "BigFile structure"}
          </span>
          <span className="explorer-sort second-header">
            <SortButton
              onClick={sortButtonPress}
              id={0}
              name="Block"
              sort={sort}
              sortBackward={sortBackward}
            />
            <SortButton
              onClick={sortButtonPress}
              id={1}
              name="Name"
              sort={sort}
              sortBackward={sortBackward}
            />
            <SortButton
              onClick={sortButtonPress}
              id={2}
              name="Extension"
              sort={sort}
              sortBackward={sortBackward}
            />
          </span>
          <div className="bffobject-list">
            <BFFObjects
              bffObjects={bigfile.objects}
              onClick={setBFFObject}
              sort={sort}
              sortBackward={sortBackward}
            />
          </div>
        </div>
        <div className="preview">
          <span className="preview-header">
            {currentBFFObject !== null
              ? currentBFFObject.name
              : "Object preview"}
          </span>
          <div>
            <span
              className={
                "second-header" +
                (openPreviewTab == 0
                  ? " preview-tabs-small"
                  : " preview-tabs-big")
              }
            >
              <button
                onClick={() => setOpenPreviewTab(0)}
                disabled={currentBFFObject === null}
                className={openPreviewTab == 0 ? "selected-tab" : ""}
              >
                Data
              </button>
              <button
                onClick={() => setOpenPreviewTab(1)}
                disabled={currentBFFObject === null}
                className={openPreviewTab == 1 ? "selected-tab" : ""}
              >
                Preview
              </button>
              <button
                onClick={() => setOpenPreviewTab(2)}
                disabled={currentBFFObject === null}
                className={openPreviewTab == 2 ? "selected-tab" : ""}
              >
                Error
              </button>
            </span>
          </div>
          {currentBFFObject !== null && (
            <div className="preview-inner">
              <PreviewContainer
                openTab={openPreviewTab}
                previewObject={currentBFFObject}
              />
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
