import { ReactNode } from "react";

const Layout: React.FC<{ children: ReactNode }> = ({ children }) => (
  <main className="flex flex-col">
    <div className="flex flex-1 overflow-hidden">{children}</div>
  </main>
);

export default Layout;
