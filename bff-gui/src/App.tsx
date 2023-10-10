import { useState } from "react";

import { View } from "./components/View";
import { Explorer } from "./components/Explorer";
import { Menubar } from "./components/Menubar";

import { BigFileData, Nickname, ResourcePreview, ViewTab } from "./types/types";

function App() {
  const [bigfile, setBigfile] = useState<BigFileData>({
    filename: "",
    resource_infos: [],
  });
  const [resourcePreview, setResourcePreview] =
    useState<ResourcePreview | null>(null);
  const [openTab, setOpenTab] = useState<ViewTab>(ViewTab.Preview);
  const [nicknames, setNicknames] = useState<Nickname[]>([]);
  const [currentNickname, setCurrentNickname] = useState<string>("");

  return (
    <div className="container">
      <Menubar
        bigfileLoaded={bigfile.filename !== ""}
        resourcePreview={resourcePreview}
        setResourcePreview={setResourcePreview}
        setBigfile={setBigfile}
      />
      <div className="main">
        <Explorer
          bigfile={bigfile}
          nicknames={nicknames}
          setResourcePreview={setResourcePreview}
          setOpenTab={setOpenTab}
          setCurrentNickname={setCurrentNickname}
        />
        <View
          resourcePreview={resourcePreview}
          nicknames={nicknames}
          currentNickname={currentNickname}
          openTab={openTab}
          setOpenTab={setOpenTab}
          setNicknames={setNicknames}
          setCurrentNickname={setCurrentNickname}
        />
      </div>
    </div>
  );
}

export default App;
