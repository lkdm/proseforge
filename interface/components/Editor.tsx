import React from "react"
import InnerEditor from '@md/markdown-editor/components/Editor'

interface EditorProps {
  content: string
}

const Editor = (props: EditorProps) => {
  return <InnerEditor {...props} />
}

export default Editor
