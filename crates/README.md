The `crates` directory keeps all the gosub crates that is currently used by the engine.
Each crate contains a specific set of code that can be used by other crates if needed.

In order to keep things organized and to make sure we don't hit dependency cycles, we 
have a crate with shared code (`gosub_shared`) that keeps generic code and all traits.

This way allows us to create a system where crates can easily use functionality from 
other crates (for instance, the HTML5 parser from `gosub_html5` uses the CSS3 parser 
from `gosub_css3`), without being directly dependend on the crate itself. This way, the 
`gosub_html5` crate can use css3 functionality, and the `gosub_css3` crate can use html5 
functionality without having a dependency on each other.

The following crates are currently available:

- `gosub_css3`: CSS3 parser and style engine.
- `gosub_html5`: HTML5 parser and document definitions.
- `gosub_renderer`: Rendering functionality.
- `gosub_shared`: Shared code and traits.
