import React from "react"
import InnerEditor from '@md/markdown-editor/components/Editor'

interface EditorProps {
  chunks: string[],
  handleSetChunk: (index: number, content: string) => void,
}

const Editor = ({chunks, handleSetChunk}: EditorProps) => {
  const [focusedNode, setFocusedNode] = React.useState<number | undefined>(undefined)
  return chunks.map((chunk, index: number) => (
    (focusedNode === index)
    ? <InnerEditor
        key={index}
        defaultContent={chunk}
        setContent={(content: string) => handleSetChunk(index, content)}
      />
      : <div onClick={() => setFocusedNode(index)} key={index}>{chunk}</div>

  ))
}

export default Editor
