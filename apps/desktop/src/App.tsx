import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Layout from "@md/interface/app/Layout"
import Content from "@md/interface/app/Content"
import Editor from "@md/interface/components/Editor"
import Document from "@md/interface/app/Document"
import { ThemeProvider } from "@md/interface/providers/ThemeProvider"
import "./App.css";
import { listen } from '@tauri-apps/api/event';
import debounce from 'lodash/debounce';

interface Config {
  theme: 'system' | 'light' | 'dark'
}

function App() {
  const [content, setContent] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [config, setConfig] = useState<Config | null>(null);

  // handle_open_dialog,
  // get_config,
  // handle_update_content

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

  useEffect(() => {
    console.log("Mounting app.")
    getConfig()
  }, [])

  listen('file-opened', (event) => {
    setIsLoading(true)
    console.log("File opened:", event.payload)
    setContent(event.payload)
    setIsLoading(false)
  });

  const handleUpdate = debounce((content: string) => {
    setContent(content);
    invoke("handle_update_content", { content })
  }, 150)

  if (!config) return <div>Loading...</div>

  return (
    <ThemeProvider defaultTheme={config.theme}>
    <Layout>
      <Content>
          <Document>
        {!isLoading && <Editor defaultContent={content} setContent={handleUpdate} />}
          </Document>
      </Content>
    </Layout>
    </ThemeProvider>
  );
}

export default App;
