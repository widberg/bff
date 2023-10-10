import { convertFileSrc } from "@tauri-apps/api/tauri";
import { TransformWrapper, TransformComponent } from "react-zoom-pan-pinch";
import { ColladaLoader } from "three/examples/jsm/loaders/ColladaLoader";
import { Canvas, useLoader } from "@react-three/fiber";
import { OrbitControls, Grid } from "@react-three/drei";
import {
  DoubleSide,
  FrontSide,
  Material,
  MeshStandardMaterial,
} from "three";
import { useState } from "react";
// import parse from "html-react-parser";

import "./View.css";

import {MeshMaterial, ResourcePreview, ViewTab, MaterialType} from "../types/types";
import {DEFAULT_MAT, IMAGE_EXT, MESH_EXT, NORMAL_MAT, SOUND_EXT, WIREFRAME_MAT} from "../constants/constants";

function PreviewInner({ previewPath }: { previewPath: string }) {
  const [rendering, setRendering] = useState<string>("pixelated");
  const [material, setMaterial] = useState<MeshMaterial>({
    name: "default",
    material: new MeshStandardMaterial(),
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

  if (previewPath.endsWith(IMAGE_EXT)) {
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
  } else if (previewPath.endsWith(SOUND_EXT))
    return (
      <div className="preview-display">
        <audio controls src={previewPath} />
      </div>
    );
  else if (previewPath.endsWith(MESH_EXT)) {
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
          camera={{ fov: 70, position: [0, 0, 5] }}
          resize={{ scroll: false, debounce: { scroll: 0, resize: 0 } }}
        >
          <OrbitControls rotateSpeed={0.7} dampingFactor={0.1} makeDefault />
          <ambientLight intensity={0.1} />
          <directionalLight color="white" position={[10, 10, 10]} />
          <directionalLight color="white" position={[-10, -10, -10]} />
          <Grid
            infiniteGrid={true}
            fadeDistance={10}
            cellColor="#444444"
            sectionColor="#888888"
          />
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
        <p>{previewObject.preview_data}</p>
      </div>
    );
  if (openTab == ViewTab.Preview) {
    if (previewObject.preview_path !== null)
      return (
        <PreviewInner
          previewPath={convertFileSrc(previewObject.preview_path as string)}
        />
      );
  }
  // if (openTab == PreviewTab.Error) {
  //   if (previewObject.error)
  //     return <p className="preview-text">{parse(previewObject.error)}</p>;
  // }
  return <></>;
}

export function View({
  resourcePreview,
  openTab,
  setOpenTab,
}: {
  resourcePreview: ResourcePreview | null;
  openTab: ViewTab;
  setOpenTab: React.Dispatch<React.SetStateAction<ViewTab>>;
}) {
  return (
    <div className="preview">
      <span className="preview-header">
        {resourcePreview !== null ? resourcePreview.name : "Object preview"}
      </span>
      <div>
        <span
          className={
            "second-header" +
            (openTab == 0 ? " tabs-small" : " tabs-big")
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
              resourcePreview === null || resourcePreview?.preview_path === null
            }
            className={
              openTab == ViewTab.Preview ? "selected-tab" : ""
            }
          >
            Preview
          </button>
          {/*<button*/}
          {/*  onClick={() => setOpenPreviewTab(PreviewTab.Error)}*/}
          {/*  disabled={previewObject === null || previewObject?.error === null}*/}
          {/*  className={openPreviewTab == PreviewTab.Error ? "selected-tab" : ""}*/}
          {/*>*/}
          {/*  Error*/}
          {/*</button>*/}
        </span>
      </div>
      {resourcePreview !== null && (
        <div className="preview-inner">
          <PreviewContainer
            openTab={openTab}
            previewObject={resourcePreview}
          />
        </div>
      )}
    </div>
  );
}
