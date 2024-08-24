import React, { ReactNode } from "react";

const Document: React.FC<{ children: ReactNode }> = ({ children }) => (
  <section className="flex justify-center">
    <div className="max-w-2xl w-full min-w-80">
      <article className="prose dark:prose-invert">{children}</article>
    </div>
  </section>
);

export default Document;
