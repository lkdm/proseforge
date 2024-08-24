import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Layout from "@md/interface/app/Layout"
import Content from "@md/interface/app/Content"
import Editor from "@md/interface/components/Editor"
import Document from "@md/interface/app/Document"
import { ThemeProvider } from "@md/interface/providers/ThemeProvider"
import "./App.css";
import { listen } from '@tauri-apps/api/event';

interface Config {
  theme: 'system' | 'light' | 'dark'
}

function App() {
  const [content, setContent] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [config, setConfig] = useState<Config | null>(null);

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  // async function load() {
  //   setContent(await invoke("load"))
  // }
  //

  async function getConfig() {
    try {
      const config = await invoke("get_config");
      setConfig({
        ...config,
        theme: config.theme.toLowerCase() as Config['theme'],
      })
      console.log("Config loaded:", config)
    } catch (error) {
      console.error("Error loading config:", error);
    }
  }

  async function load() {
    setIsLoading(true)
      try {
        const result = await invoke("load");
        setContent(result as string);
        console.log("Content loaded:", result)
      } catch (error) {
        setError(JSON.stringify(error));
        console.error("Error loading content:", error);
      }
      setIsLoading(false);
    }

  useEffect(() => {
    console.log("Mounting app.")
    getConfig()
    load()
  }, [])

  listen('file-opened', (event) => {
    load();
  });


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
      console.log("Content saved:", content)
    } catch (error) {
      console.error("Error saving content:", error);
    }
  }

  listen('file-save', () => {
    handleSave();
  })

  if (!config) return <div>Loading...</div>

  return (
    <ThemeProvider defaultTheme={config.theme}>
    <Layout>
      <Content>
          <Document>
        {!isLoading && <Editor defaultContent={content} setContent={setContent} />}
          </Document>
      </Content>
    </Layout>
    </ThemeProvider>
  );
}

export default App;
