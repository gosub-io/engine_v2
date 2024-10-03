The average life of a request coming into a user-agent (browser) works like this:

 - The user agent receives a request to goto an url and loads the document from the remote webserver. 
 - The engine parses creates a `Document` through a `DocumentHandle`, and passes the handle and the HTML source to the html5 parser.
 - The html5 parser parses the document and creates a tree of nodes in `Document`. When it encounters CSS, it will also generate `Csstylesheets` that are located in the `Document`
 - After the html5 parsing, the user agent creates a `RenderTree` that will generate a structure that tells which nodes have which CSS properties.
 - The `RenderTree` is then passed to the `Layouter` to generate a layout.
 - ***??? ... Something something Treedrawer stuff ... ***
 - And finally, the Layouter is given to the RenderBackend backend that can actually paint the layout on the screen.
