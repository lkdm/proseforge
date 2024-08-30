
-- A project is a collection of components.
CREATE TABLE IF NOT EXISTS project (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    kind TEXT NOT NULL, -- ie. "novel" or "shortstory"

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    modified_at TIMESTAMP,
    deleted_at TIMESTAMP
);

-- A component is a part of a project. It can be a directory or a document.
-- It dictates how the project is structured.
CREATE TABLE IF NOT EXISTS project_component (
    id INTEGER PRIMARY KEY,
    kind TEXT NOT NULL,
    display_order INTEGER NOT NULL DEFAULT 0 CHECK (display_order >= 0),

    title TEXT,
    summary TEXT,

    project_id INTEGER NOT NULL,
    parent_id INTEGER, -- null, if it's a root node
    document_id INTEGER, -- null, if it's a directory

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    modified_at TIMESTAMP,
    deleted_at TIMESTAMP,

    UNIQUE (document_id), -- A document can only be owned by one component.
    FOREIGN KEY (parent_id) REFERENCES project_component(id) ON DELETE CASCADE,
    FOREIGN KEY (document_id) REFERENCES editor_document(id) ON DELETE SET NULL, -- If a document is deleted, the component turns into a directory.
    FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);

-- A document is anything that can be consumed by the editor.
CREATE TABLE IF NOT EXISTS editor_document (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,

    content TEXT,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    modified_at TIMESTAMP,
    deleted_at TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);
