import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Layout from "@md/interface/app/Layout"
import Content from "@md/interface/app/Content"
import Editor from "@md/interface/components/editor/Editor"
import "./App.css";

function App() {
  const [content, setContent] = useState("");
  const [error, setError] = useState<string | null>(null);

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  // async function load() {
  //   setContent(await invoke("load"))
  // }

  async function load() {
      try {
        const result = await invoke("load");
        setContent(result as string);
        console.log("Content loaded:", result)
      } catch (error) {
        setError(JSON.stringify(error));
        console.error("Error loading content:", error);
      }
    }

  useEffect(() => {
    console.log("Mounting app.")
    load()
  }, [])

  return (
    <Layout>
      <Content>
        <Editor content={content} />
      </Content>
    </Layout>
  );
}

export default App;
