type DocumentId = u32;

#[derive(Debug, Clone)]
struct Document {
    id: DocumentId,
    content: String,
    is_modified: bool,
}

#[derive(Debug, Clone)]
enum DraftNode {
    Leaf {
        title: String,
        summary: String,
        content: Document,
    },
    Branch {
        title: String,
        summary: String,
        children: Vec<DraftNode>,
    },
}

#[derive(Debug, Clone)]
enum NoteNode {
    Leaf {
        title: String,
        content: Document,
    },
    Branch {
        title: String,
        children: Vec<NodeNode>,
    },
}

struct Draft {
    children: Vec<DraftNode>,
}

// Use Node to generic enum.
enum Node<T> {
    Branch {
        value: T,
        children: Vec<Box<Node<T>>>,
    },
    Leaf(T),
}
