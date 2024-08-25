import { defaultValueCtx, Editor, rootCtx } from '@milkdown/kit/core';
import { nord } from '@milkdown/theme-nord';
import { Milkdown, MilkdownProvider, useEditor } from '@milkdown/react';
import { listener, listenerCtx } from "@milkdown/plugin-listener";
import { history } from '@milkdown/plugin-history';
import { clipboard } from '@milkdown/kit/plugin/clipboard';
import { indent } from '@milkdown/kit/plugin/indent';
import { commonmark } from '@milkdown/kit/preset/commonmark';

interface EditorProps {
  defaultContent: string;
  setContent: (content: string) => void;
  eventTimestamp: number | null;
}

const MilkdownEditor = ({defaultContent, setContent, eventTimestamp}: EditorProps) => {

    useEditor((root) =>
      Editor.make()
        .config(nord)
        .config((ctx) => {
          ctx.set(rootCtx, root)
          ctx.set(defaultValueCtx, defaultContent)
          ctx
            .get(listenerCtx)
            // .updated((ctx, doc, prevDoc) => {
              // console.log("updated", doc, prevDoc);
            // })
            .markdownUpdated((_, markdown) => {
                setContent(markdown);
              })
            .destroy(() => console.log("Destroyed"))
        })
        .use(commonmark)
        .use(history)
        .use(clipboard)
        .use(indent)
        .use(listener),
      [eventTimestamp]
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
