import React, { useEffect, useState } from "react"
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
  MarkdownSerializer as ProseMirrorMarkdownSerializer,
  defaultMarkdownSerializer,
} from "prosemirror-markdown";
import { DOMParser as ProseMirrorDOMParser } from "prosemirror-model";
import {marked} from "marked";

import {
  useEditor,
  EditorContent,
  JSONContent,
  // FloatingMenu,
  // BubbleMenu,
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

const serializerMarks = {
  ...defaultMarkdownSerializer.marks,
  [Bold.name]: defaultMarkdownSerializer.marks.strong,
  [Strike.name]: {
    open: "~~",
    close: "~~",
    mixable: true,
    expelEnclosingWhitespace: true,
  },
  [Italic.name]: {
    open: "_",
    close: "_",
    mixable: true,
    expelEnclosingWhitespace: true,
  },
};

const serializerNodes = {
  ...defaultMarkdownSerializer.nodes,
  [Paragraph.name]: defaultMarkdownSerializer.nodes.paragraph,
  [BulletList.name]: defaultMarkdownSerializer.nodes.bullet_list,
  [ListItem.name]: defaultMarkdownSerializer.nodes.list_item,
  [HorizontalRule.name]: defaultMarkdownSerializer.nodes.horizontal_rule
};

function serialise(schema: any, content: JSONContent) {
  const proseMirrorDocument = schema.nodeFromJSON(content);
  const serializer = new ProseMirrorMarkdownSerializer(
    serializerNodes,
    serializerMarks
  );

  return serializer.serialize(proseMirrorDocument, {
    tightLists: true,
  });
}

function deserialise(schema: any, content: string) {
  const html = marked.parse(content);

  if (!html) return null;

  const parser = new DOMParser();
  const { body } = parser.parseFromString(html, "text/html");

  // append original source as a comment that nodes can access
  body.append(document.createComment(content));

  const state = ProseMirrorDOMParser.fromSchema(schema).parse(body);

  return state.toJSON();
}

interface Props {
  defaultContent: string;
  handleContentChange: (content: string) => void;
}

const Tiptap = ({ defaultContent, handleContentChange }: Props) => {

  const editor = useEditor({
    extensions,
    content: loadMarkdownInput(defaultContent),
    onCreate({ editor }) {
      handleContentChange(serialise(editor.schema, editor.getJSON()));
    },
    onUpdate: ({ editor }) => {
      handleContentChange(serialise(editor.schema, editor.getJSON()));
    },
    editorProps: {
      attributes: {
        class:
          "prose dark:prose-invert prose-lg prose-serif focus:outline-none",
      },
    },
  });

  function loadMarkdownInput(content: string) {
     const deserialized = deserialise(editor?.schema, content);
     editor?.commands.setContent(deserialized);
     return deserialized;
   }


  return <EditorContent editor={editor} />
};

export default Tiptap;
