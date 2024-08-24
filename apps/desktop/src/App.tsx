import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Layout from "@md/interface/app/Layout"
import Content from "@md/interface/app/Content"
import Editor from "@md/interface/components/Editor"
import Document from "@md/interface/app/Document"
import { ThemeProvider } from "@md/interface/providers/ThemeProvider"
import "./App.css";
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import debounce from 'lodash/debounce';

interface Config {
  theme: 'system' | 'light' | 'dark'
}

function App() {
  const [content, setContent] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [config, setConfig] = useState<Config | null>(null);
  const [eventTimestamp, setEventTimestamp] = useState<number>(0);

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


  useEffect(() => {
    let unlistenFunction: UnlistenFn | null = null;

    async function setupListener() {
      unlistenFunction = await listen('file-opened', (event) => {
        setIsLoading(true);
        const { content: newContent, timestamp } = event.payload.DocumentLoad as {
            content: string;
            timestamp: number;
          };
        console.log(timestamp, newContent)
        setContent(newContent || '');
        setEventTimestamp(timestamp);
        setIsLoading(false);
      });
    }

    setupListener();

    // Cleanup listener on unmount
    return () => {
      if (unlistenFunction) {
        unlistenFunction();
      }
    };
  }, [eventTimestamp]);

  const handleUpdate = debounce((content: string) => {
    setContent(content);
    invoke("handle_update_content", { content })
  }, 150)

  useEffect(() => {
      console.log("Editor re-rendered with timestamp:", eventTimestamp);
    }, [eventTimestamp]);

  useEffect(() => {
    console.log("isLoading changed:", isLoading);
  }, [isLoading]);

  if (!config) return <div>Loading...</div>

  return (
    <ThemeProvider defaultTheme={config.theme}>
    <Layout>
      <Content>
          <Document>
        {isLoading
        ? <>Loading...</>
              : <Editor defaultContent={content} setContent={handleUpdate} eventTimestamp={eventTimestamp}  />}
          </Document>
      </Content>
    </Layout>
    </ThemeProvider>
  );
}

export default App;
