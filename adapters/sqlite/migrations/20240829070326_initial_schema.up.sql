
-- A project is a collection of components.
CREATE TABLE IF NOT EXISTS project (
    id BLOB PRIMARY KEY,
    title TEXT NOT NULL,
    kind TEXT NOT NULL, -- ie. "novel" or "shortstory"

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    modified_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS component_summary (
    id BLOB PRIMARY KEY,
    content TEXT,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    modified_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP
);


-- A document is anything that can be consumed by the editor.
CREATE TABLE IF NOT EXISTS document (
    id BLOB PRIMARY KEY,
    project_id BLOB NOT NULL,

    content TEXT,
    word_count INTEGER NOT NULL DEFAULT 0 CHECK (word_count >= 0),

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    modified_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);


-- A component is a part of a project. It can be a directory or a document.
-- It dictates how the project is structured.
CREATE TABLE IF NOT EXISTS component (
    id BLOB PRIMARY KEY,
    kind TEXT NOT NULL,
    display_order INTEGER NOT NULL DEFAULT 0 CHECK (display_order >= 0),

    title TEXT,

    project_id BLOB NOT NULL,
    path TEXT NOT NULL, -- e.g., "/root/parent/child"

    document_id BLOB, -- null, if it's a directory
    summary_id BLOB,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    modified_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP,

    UNIQUE (document_id), -- A document can only be owned by one component.
    UNIQUE (summary_id), -- A summary can only be owned by one component.
    FOREIGN KEY (document_id) REFERENCES document(id) ON DELETE SET NULL, -- If a document is deleted, the component turns into a directory.
    FOREIGN KEY (summary_id) REFERENCES component_summary(id) ON DELETE SET NULL,
    FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);
