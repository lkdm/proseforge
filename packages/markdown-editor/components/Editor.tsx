import React, { useEffect, useState } from 'react';
import { defaultValueCtx, Editor, rootCtx, editorViewCtx } from '@milkdown/kit/core';
import { nord } from '@milkdown/theme-nord';
import { Milkdown, MilkdownProvider, useEditor } from '@milkdown/react';
import { listener, listenerCtx } from "@milkdown/plugin-listener";
import { history } from '@milkdown/plugin-history';
import { clipboard } from '@milkdown/kit/plugin/clipboard';
import { indent } from '@milkdown/kit/plugin/indent';
import { commonmark } from '@milkdown/kit/preset/commonmark';

// TOOD: https://codesandbox.io/p/sandbox/react-grdxqn?file=%2Fsrc%2FApp.js%3A98%2C9-98%2C23

interface EditorProps {
  defaultContent: string;
  setContent: (content: string) => void;
}

const MilkdownEditor = ({defaultContent, setContent}: EditorProps) => {

    const { get } = useEditor((root) =>
      Editor.make()
        .config(nord)
        .config((ctx) => {
          ctx.set(rootCtx, root)
          ctx.set(defaultValueCtx, defaultContent)
          ctx
            .get(listenerCtx)
            .mounted((ctx) => {
              // Focus the editor
              const view = ctx.get(editorViewCtx);
              view.focus();
            })
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
        .use(history)
        .use(clipboard)
        .use(indent)
        .use(listener)
  );

  return <Milkdown />;
};

const MilkdownEditorWrapper = (props: EditorProps) => {
  return (
    <MilkdownProvider>
      <MilkdownEditor { ...props } />
    </MilkdownProvider>
  );
};

export default MilkdownEditorWrapper
