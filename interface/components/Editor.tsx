import React from "react"
import InnerEditor from '@md/markdown-editor/components/Editor'

interface EditorProps {
  defaultContent: string,
  setContent: (content: string) => void,
  eventTimestamp: number | null
}

const Editor = (props: EditorProps) => {
  return <InnerEditor {...props} />
}

export default Editor
