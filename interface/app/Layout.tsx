import React, { ReactNode } from "react";

const Layout: React.FC<{ children: ReactNode }> = ({ children }) => (
  <main className="flex flex-col h-screen">
    <div className="flex flex-1 overflow-hidden">{children}</div>
  </main>
);

export default Layout;
