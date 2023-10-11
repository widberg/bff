import { TransformWrapper, TransformComponent } from "react-zoom-pan-pinch";
import { ColladaLoader } from "three/examples/jsm/loaders/ColladaLoader";
import { Canvas } from "@react-three/fiber";
import { OrbitControls, Grid } from "@react-three/drei";
import { DoubleSide, FrontSide, Material } from "three";
import { useState } from "react";
// import parse from "html-react-parser";

import "./View.css";

import {
  MeshMaterial,
  ResourcePreview,
  ViewTab,
  MaterialType,
  PreviewData,
  DataType,
  Nickname,
} from "../types/types";
import { DEFAULT_MAT, NORMAL_MAT, WIREFRAME_MAT } from "../constants/constants";

function PreviewInner({ previewData }: { previewData: PreviewData }) {
  const [rendering, setRendering] = useState<string>("pixelated");
  const [material, setMaterial] = useState<MeshMaterial>({
    name: "default",
    material: DEFAULT_MAT,
  });

  function setFilter(enabled: boolean) {
    setRendering(enabled ? "auto" : "pixelated");
  }
  function changeMaterial(type: string) {
    let mat: Material = DEFAULT_MAT;
    if (type == MaterialType.Normal) mat = NORMAL_MAT;
    else if (type == MaterialType.Wireframe) mat = WIREFRAME_MAT;
    mat.side = material.material.side;
    setMaterial({
      name: type,
      material: mat,
    });
  }
  function setSide(checked: boolean) {
    material.material.side = checked ? DoubleSide : FrontSide;
  }

  if (previewData.data_type == DataType.Image) {
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
            src={"data:image/png;base64," + previewData.data}
            alt="image"
            className="preview-image"
          />
        </TransformComponent>
      </TransformWrapper>
    );
  } else if (previewData.data_type == DataType.Sound)
    return (
      <div className="preview-display">
        <audio controls src={"data:audio/wav;base64," + previewData.data} />
      </div>
    );
  else if (previewData.data_type == DataType.Mesh) {
    const scene = new ColladaLoader().parse(previewData.data, "/").scene;

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
            <option value={MaterialType.Default}>Default</option>
            <option value={MaterialType.Normal}>Normal</option>
            <option value={MaterialType.Wireframe}>Wireframe</option>
          </select>
          <div>
            <label htmlFor="sided">No culling</label>
            <input
              type="checkbox"
              id="sided"
              defaultChecked={material.material.side == DoubleSide}
              onChange={(e) => setSide(e.target.checked)}
            />
          </div>
        </div>
        <Canvas
          camera={{ fov: 70, position: [0, 5, 5] }}
          resize={{ scroll: false, debounce: { scroll: 0, resize: 0 } }}
        >
          <OrbitControls rotateSpeed={0.7} dampingFactor={0.1} makeDefault />
          <ambientLight intensity={0.1} />
          <directionalLight color="white" position={[10, 10, 10]} />
          <directionalLight color="white" position={[-10, -10, -10]} />
          <Grid
            infiniteGrid={true}
            fadeDistance={30}
            cellColor="#444444"
            sectionColor="#888888"
          />
          <group>
            <primitive object={scene} children-0-material={material.material} />
          </group>
        </Canvas>
      </div>
    );
  } else if (previewData.data_type == DataType.Text)
    return (
      <div className="preview-data preview-text">
        <p>{previewData.data}</p>
      </div>
    );
  else {
    return <></>;
  }
}

function PreviewContainer({
  openTab,
  previewObject,
}: {
  openTab: number;
  previewObject: ResourcePreview;
}) {
  if (openTab == ViewTab.Data)
    return (
      <div className="preview-data preview-text">
        <p>{previewObject.preview_json}</p>
      </div>
    );
  if (openTab == ViewTab.Preview) {
    if (previewObject.preview_data !== null)
      return (
        <PreviewInner previewData={previewObject.preview_data as PreviewData} />
      );
  }
  return <></>;
}

function NicknameInput({
  resourcePreview,
  nicknames,
  currentNickname,
  setNicknames,
  setCurrentNickname,
}: {
  resourcePreview: ResourcePreview;
  nicknames: Nickname[];
  currentNickname: string;
  setNicknames: React.Dispatch<React.SetStateAction<Nickname[]>>;
  setCurrentNickname: React.Dispatch<React.SetStateAction<string>>;
}) {
  return (
    <input
      className="nicknamebox"
      type="text"
      placeholder="Enter nickname..."
      value={currentNickname}
      onChange={(e) => setCurrentNickname(e.target.value)}
      onBlur={() => {
        if (currentNickname !== "")
          setNicknames([
            ...nicknames.filter((n) => {
              return n.name !== resourcePreview.name;
            }),
            {
              name: resourcePreview.name,
              nickname: currentNickname,
            },
          ]);
        else
          setNicknames([
            ...nicknames.filter((n) => {
              return n.name !== resourcePreview.name;
            }),
          ]);
      }}
    ></input>
  );
}

export function View({
  resourcePreview,
  nicknames,
  currentNickname,
  openTab,
  setOpenTab,
  setNicknames,
  setCurrentNickname,
}: {
  resourcePreview: ResourcePreview | null;
  nicknames: Nickname[];
  currentNickname: string;
  openTab: ViewTab;
  setOpenTab: React.Dispatch<React.SetStateAction<ViewTab>>;
  setNicknames: React.Dispatch<React.SetStateAction<Nickname[]>>;
  setCurrentNickname: React.Dispatch<React.SetStateAction<string>>;
}) {
  return (
    <div className="preview">
      <span className="preview-header">
        {resourcePreview !== null && (
          <NicknameInput
            resourcePreview={resourcePreview}
            nicknames={nicknames}
            currentNickname={currentNickname}
            setNicknames={setNicknames}
            setCurrentNickname={setCurrentNickname}
          />
        )}
        {resourcePreview !== null ? resourcePreview.name : "Object preview"}
      </span>
      <div>
        <span
          className={
            "second-header" +
            (openTab == ViewTab.Data ||
            resourcePreview?.preview_data?.data_type == DataType.Text
              ? " tabs-small"
              : " tabs-big")
          }
        >
          <button
            onClick={() => setOpenTab(ViewTab.Data)}
            disabled={resourcePreview === null}
            className={openTab == ViewTab.Data ? "selected-tab" : ""}
          >
            Data
          </button>
          <button
            onClick={() => setOpenTab(ViewTab.Preview)}
            disabled={
              resourcePreview === null || resourcePreview?.preview_data === null
            }
            className={openTab == ViewTab.Preview ? "selected-tab" : ""}
          >
            Preview
          </button>
        </span>
      </div>
      {resourcePreview !== null && (
        <div className="preview-inner">
          <PreviewContainer openTab={openTab} previewObject={resourcePreview} />
        </div>
      )}
    </div>
  );
}
