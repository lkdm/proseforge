import React from "react";
import { Paragraph } from "@tiptap/extension-paragraph";
import { Blockquote } from "@tiptap/extension-blockquote";
import { BulletList } from "@tiptap/extension-bullet-list";
import { OrderedList } from "@tiptap/extension-ordered-list";
import { ListItem } from "@tiptap/extension-list-item";
import { ListKeymap } from "@tiptap/extension-list-keymap";
import { Heading } from "@tiptap/extension-heading";
import { HorizontalRule } from "@tiptap/extension-horizontal-rule";
import { Bold } from "@tiptap/extension-bold";
import { Italic } from "@tiptap/extension-italic";
import { Strike } from "@tiptap/extension-strike";
import { Underline } from "@tiptap/extension-underline";
import { Typography } from "@tiptap/extension-typography";
import { History } from "@tiptap/extension-history";
import { Document } from "@tiptap/extension-document";
import { Text } from "@tiptap/extension-text";
import { HardBreak } from "@tiptap/extension-hard-break";
import Link from "@tiptap/extension-link";
import {
  useEditor,
  EditorContent,
  FloatingMenu,
  BubbleMenu,
} from "@tiptap/react";

// define your extension array
const extensions = [
  Document,
  Paragraph,
  Text,
  HardBreak,
  BulletList,
  OrderedList,
  ListItem,
  ListKeymap,
  Blockquote,
  Heading,
  HorizontalRule,
  Bold,
  Italic,
  Strike,
  Underline,
  Typography,
  History,
  Link.configure({
    openOnClick: false,
    autolink: true,
  }),
];

interface Props {
  content: string;
}

const Tiptap = ({ content }: Props) => {
  const editor = useEditor({
    extensions,
    content,
    onUpdate: ({ editor }) => {
      console.log("Editor content:", editor.getHTML());
    },
    editorProps: {
      attributes: {
        class:
          "prose dark:prose-invert prose-lg prose-serif focus:outline-none",
      },
    },
  });

  return <EditorContent editor={editor} />
};

export default Tiptap;
