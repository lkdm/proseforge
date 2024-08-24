import React, { ReactNode, useRef } from "react";
import useFocusEditor from "../hooks/useFocusEditor";

const Document: React.FC<{ children: ReactNode }> = ({ children }) => {
  const { handleButtonClick, handleArticleClick } = useFocusEditor()

  return (
  <section className="flex justify-center h-full" onClick={handleButtonClick}>
    <div className="max-w-2xl w-full min-w-80 h-full">
      <article className="prose dark:prose-invert h-full" onClick={handleArticleClick}>{children}</article>
    </div>
  </section>
)};

export default Document;
