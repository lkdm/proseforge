import React, {useEffect, useCallback} from "react"
import InnerEditor from '@md/markdown-editor/components/Editor'

interface EditorProps {
  chunks: string[],
  handleSetChunk: (index: number, content: string) => void,
}

// Editor should only re-render if the focusedNode changes
const MemoizedInnerEditor = React.memo(InnerEditor);

const Editor = ({chunks, handleSetChunk}: EditorProps) => {
  const [focusedNode, setFocusedNode] = React.useState<number | undefined>(undefined)

  const handleKeyDown = useCallback((event: KeyboardEvent) => {
    if (typeof focusedNode !== 'number') return;
    switch (event.key) {
      case 'ArrowUp':
        if (focusedNode > 0) setFocusedNode(focusedNode - 1);
        break;
      case 'ArrowDown':
        if (focusedNode < chunks.length - 1) setFocusedNode(focusedNode + 1);
        break;
      default:
        break;
    }
  }, [focusedNode, setFocusedNode, chunks.length]);

  useEffect(() => {
    // Add keydown event listener on mount
    window.addEventListener('keydown', handleKeyDown);
    // Cleanup event listener on unmount
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [handleKeyDown]);

  return chunks.map((chunk, index: number) => (
    (focusedNode === index)
    ? <MemoizedInnerEditor
        key={index}
        defaultContent={chunk}
        setContent={(content: string) => handleSetChunk(index, content)}
      />
      : <div onClick={() => setFocusedNode(index)} key={index}>{chunk}</div>

  ))
}

export default Editor
