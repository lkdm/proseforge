import React from "react"
import InnerEditor from '@md/markdown-editor/components/Editor'

interface EditorProps {
  defaultContent: string,
  setContent: (content: string) => void,
}

const Editor = (props: EditorProps) => {
  return <InnerEditor {...props} />
}

export default Editor
