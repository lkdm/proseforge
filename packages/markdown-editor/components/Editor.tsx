import React, { useState } from 'react';
import { defaultValueCtx, Editor, rootCtx } from '@milkdown/kit/core';
import { nord } from '@milkdown/theme-nord';
import { Milkdown, MilkdownProvider, useEditor } from '@milkdown/react';
import { listener, listenerCtx } from "@milkdown/plugin-listener";
import { commonmark } from '@milkdown/kit/preset/commonmark';

// TOOD: https://codesandbox.io/p/sandbox/react-grdxqn?file=%2Fsrc%2FApp.js%3A98%2C9-98%2C23

const MilkdownEditor: React.FC = () => {
  const [content, setContent] = useState("# hello \nSelect me to annotate me!");

    const { get } = useEditor((root) =>
      Editor.make()
        .config(nord)
        .config((ctx) => {
          ctx.set(rootCtx, root)
          ctx.set(defaultValueCtx, content)
          ctx
            .get(listenerCtx)
            .updated((ctx, doc, prevDoc) => {
              console.log("updated", doc, prevDoc);
            })
            .markdownUpdated((ctx, markdown, prevMarkdown) => {
                console.log(
                  "markdownUpdated to=",
                  markdown,
                  "\nprev=",
                  prevMarkdown
                );
                setContent(markdown);
              })
        })
        .use(commonmark)
        .use(listener)
  );

  return <Milkdown />;
};

const MilkdownEditorWrapper: React.FC = () => {
  return (
    <MilkdownProvider>
      <MilkdownEditor />
    </MilkdownProvider>
  );
};

export default MilkdownEditorWrapper
