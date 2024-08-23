import React from "react"
import InnerEditor from '@md/markdown-editor/components/Editor'

interface EditorProps {
  chunks: string[],
  handleSetChunk: (index: number, content: string) => void,
}

const Editor = ({chunks, handleSetChunk}: EditorProps) => {
  return chunks.map((chunk, index) => (
    <InnerEditor
      key={index}
      defaultContent={chunk}
      setContent={(content: string) => handleSetChunk(index, content)}
    />
  ))
}

export default Editor
