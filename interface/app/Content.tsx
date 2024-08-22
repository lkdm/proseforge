import React, { ReactNode } from "react";

const Content: React.FC<{ children: ReactNode }> = ({ children }) => (
  <main className="flex-1 p-4 card overflow-y-auto">{children}</main>
);

export default Content;
