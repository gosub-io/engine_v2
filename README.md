This repository holds a concept of the gosub_engine in a new form. It should relieve the painpoints of the current 
engine with traits / generics and a more modular design. The goal is to have a more flexible engine that can be
extended more easily.



TODO / Test cases we need to cover:
- [X] add node structure
- [X] add different node types: Document, Element, Text, Comment, DocumentType
- [X] add document structure
- [X] add arena (with node_ids)
- [X] add css node structure
- [X] add css document tree
- [X] allow user to add nodes to DOM, build a document with a DocumentBuilder
- [X] allow user to parse a html5 string and return (random) nodes in a DOM document from it.
- [X] attach CSS stylesheets to the DOM
- [ ] let user query the DOM for nodes
- [ ] let user query the DOM for nodes with a specific CSS selector
- [X] allow user that adds an attribute to a (element) node automatically set the named_ids in the document.
- [X] allow user to modify node data from document element (don't know it this is needed / feasable)
- [X] add document task queue
- [X] Allow callers to update attributes in nodes in an efficient way
- [ ] allow user to retrieve info from the node from its element data (like node_id, doc_handle)
