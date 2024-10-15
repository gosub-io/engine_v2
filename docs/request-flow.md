# Request flow through the Engine

The average life of a request coming into a user-agent (browser) works like this:

### Step 1
The user agent receives a request to goto an url and loads the document from the remote webserver. The loader stores 
the data into a bytestream reader which can be read by the engine.

### Step 2 
The engine parses creates a `Document` through a `DocumentHandle`, and passes the handle and the HTML source to the 
html5 parser.

### Step 3
The html5 parser parses the document and creates a tree of nodes in `Document`. When it encounters CSS, it will also 
generate `Csstylesheets` that are located in the `Document`.

## Step 4
After the html5 parsing, the user agent creates a `RenderTree` that will generate a structure that tells which nodes 
have which CSS properties.
 
## Step 5
The `RenderTree` is then passed to the `Layouter` to generate a (box) layout of all the (renderable) elements. This
should be a tree of boxes that can be painted on the screen.

## Step 6
 - Finally, the `Layouter` is given to the `RenderBackend` backend that can actually paint the layout on the screen.


