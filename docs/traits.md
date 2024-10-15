The engine consists of a lot of different traits that are used to define the behavior of the engine. This allows us to create a more modular design where we can easily swap out implementations of certain parts of the engine. It also allows us to fix issues that can arise when certain crates depend on each other.

A trait is usually accompanied by generics. This allows us to define the behavior of the trait in a more flexible way. For instance, the `Document` trait is defined as follows:

## ModuleConfiguration

The main configuration of the engine is done through the `ModuleConfiguration` trait. As a user agent, you will define the behavior of the engine by implementing this trait. The trait is defined as follows:

```rust 
pub trait ModuleConfiguration:
    Sized
    + HasCssSystem
    + HasDocument
    + HasCssParser
    + HasHtmlParser
    + HasLayouter
    + HasRenderTree
    + HasTreeDrawer
    + HasRenderBackend
{}
```

This tells us that the `ModuleConfiguration` is a placeholder for all kind of other systems, like a Cssparser, A Layouter, A RenderBackend, A CssSystem etc. All of these can be easily swapped out for other implementations when needed. 

For instance, it would be fairly trivial to swap a graphical rendering backend for a text-based rendering backend. Or to swap the CSS parser for a different implementation. This allows us to create a more flexible engine that can be easily extended.

## HasCssSystem
This defines the Css Stylesheet system that is used. It contains of all kind of traits like `CssStylesheet`, `CssRule`, `CssDeclaration`, `CssValue` etc. Both the CSS system and the rendering components depend heavily on this system.

### HasDocument
This trait defines the Document system that is used. It contains of all kind of traits like `Document`, `Element`, `Text`, `Comment`, `DocumentType` etc. The Document system is the core of the engine and is used by all other systems.

Internally, we use a Document (`MyDocument`) that contains nodes in an arena-like system. It is quite possible that other implementation would be more efficient, so we could theoretically swap out the `MyDocument` implementation for a different one.

### HasCssParser
The actual CSS parser that will be used by the HTML parser.

### HasHtmlParser
The actual HTML parser that will be used by the engine when parsing a HTML5 source into a document.

### HasLayouter
The layouter that will be used to generate a layout from the RenderTree.

### HasRenderTree
The RenderTree that will be used to generate a structure that tells which nodes have which CSS properties.

### HasTreeDrawer
The TreeDrawer that will be used to draw the RenderTree.

### HasRenderBackend
The RenderBackend that will be used to paint the layout on the screen.


## Example code:

```rust

// Define our module configuration that will be used by the engine 
struct MyModuleConfiguration;

// Implement all the traits that are needed for the engine
impl HasCssSystem for MyModuleConfiguration {
    // Here we define the actual CSS types. Theoretically we could simply swap out
    // a CssRule with another implementation, but for now we'll keep it simple.
    type CssStylesheet = <MyCssSystem as HasCssSystem>::CssStylesheet;
    type CssRule = <MyCssSystem as HasCssSystem>::CssRule;
    type CssDeclaration = <MyCssSystem as HasCssSystem>::CssDeclaration;
    type CssValue = <MyCssSystem as HasCssSystem>::CssValue;
}

// A document must define the node type that is used in the document.
impl HasDocument for MyModuleConfiguration {
    type Document = MyDocument<Self>;
    type Node = <Self::Document as Document<Self>>::Node;
}

/* 
  Some implementation want to have other configuration elements from the
  ModuleConfiguration. In that case, we can specify Self as a generic. The actual
  implementation will only specify the trait we need. So for instance, the MyHTMLParser
  needs a document and css parser, so it will have a generic like this:

      struct MyHtmlParser<C: HasDocument + HasCssParser> { ... }
            
 */

impl HasHtmlParser for MyModuleConfiguration {
    type HtmlParser = MyHtmlParser<Self>;
}

impl HasLayouter for MyModuleConfiguration {
    type Layouter = MyLayouter<Self>;
}

impl HasRenderTree for MyModuleConfiguration {
    type RenderTree = MyRenderTree<Self>;
}

impl HasTreeDrawer for MyModuleConfiguration {
    type TreeDrawer = MyTreeDrawer<Self>;
}

impl HasRenderBackend for MyModuleConfiguration {
    type RenderBackend = MyTextRenderBackend;
}

impl HasCssParser for MyModuleConfiguration { type CssParser = MyCss3Parser<Self>; }

impl ModuleConfiguration for MyModuleConfiguration {}

fn main() {
    main_do_things::<MyModuleConfiguration>();
}

fn main_do_things<C: ModuleConfiguration>() {

    // Since the document builder has a generic, we know it will output a document
    // as defined in the moduleconfiguration.
    let handle = DocumentBuilder::new_document("https://example.com");
    
    // We call the HTML parser from our configuration in order to parse the HTML
    let mut html_parser = C::HtmlParser::new(handle.clone());
    html_parser.parse_str("<html><head></head><body><p>Hello world!</p></body></html>");

    // Same as the DocumentBuilder, we define a generic for the walker so it can work
    // with the document as defined in the module configuration.
    let walker = DocumentWalker::new(handle.clone());
    walker.print_tree(handle.clone(), true);
}

```