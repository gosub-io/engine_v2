This repository holds a concept of the gosub_engine in a new form. It should relieve the painpoints of the current 
engine with traits / generics and a more modular design. The goal is to have a more flexible engine that can be
extended more easily.



TODO / Test cases we need to cover:
- [ ] add node structure
- [ ] add different node types: Document, Element, Text, Comment, DocumentType
- [ ] add document structure
- [ ] add arena (with node_ids)
- [ ] add css node structure
- [ ] add css document tree
- [ ] allow user to add nodes to DOM, build a document with a DocumentBuilder
- [ ] allow user to parse a html5 string and return (random) nodes in a DOM document from it.
- [ ] attach CSS stylesheets to the DOM
- [ ] let user query the DOM for nodes
- [ ] let user query the DOM for nodes with a specific CSS selector
- [ ] allow user that adds an attribute to a (element) node automatically set the named_ids in the document.
- [ ] allow user to retrieve info from the node from its element data (like node_id, doc_handle)
- [ ] allow user to modify node data from document element (don't know it this is needed / feasable)
- [ ] add document task queue (optional for now)