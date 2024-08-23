import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Layout from "@md/interface/app/Layout"
import Content from "@md/interface/app/Content"
import Editor from "@md/interface/components/Editor"
import "./App.css";

function App() {
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [chunks, setChunks] = useState<string[]>([]);

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  // async function load() {
  //   setContent(await invoke("load"))
  // }

  async function load() {
    setIsLoading(true)
      try {
        const result = await invoke("load");
        setChunks(result.split("\n"));
        console.log("Content loaded:", result)
      } catch (error) {
        setError(JSON.stringify(error));
        console.error("Error loading content:", error);
      }
      setIsLoading(false);
    }

  useEffect(() => {
    console.log("Mounting app.")
    load()
  }, [])

  const handleOpenDialogue = async () => {
    try {
      await invoke("open_file_dialogue");
      await load();
    } catch (error) {
      console.error("Error opening dialogue:", error);
    }
  }

  const handleSave = async () => {
    try {
      await invoke("save", { content });
      console.log("Content saved:", chunks.join("\n"))
    } catch (error) {
      console.error("Error saving content:", error);
    }
  }

  const handleSetChunk = (index: number, value: string) => {
    const newChunks = [...chunks];
    newChunks[index] = value;
    setChunks(newChunks);
  }

  return (
    <Layout>
      <Content>
        <button onClick={handleOpenDialogue}>Load</button>
        <button onClick={handleSave}>Save</button>
        {!isLoading && <Editor chunks={chunks} handleSetChunk={handleSetChunk} />}
      </Content>
    </Layout>
  );
}

export default App;
