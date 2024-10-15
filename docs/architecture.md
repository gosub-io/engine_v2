# Architecture

This is a rough draft on how the system looks like from a high level perspective.

```mermaid
---
title: Gosub Engine V2
---
%%{ init: { 'graph': { 'curve': 'linear' } } }%%    
graph TB    
    style document fill:#ffcccc
    subgraph document [Document]
      NodeBuilder ---> Document       
      Node ---> NodeBuilder
      Document ---> HtmlParser
      Node --> Document
    end
    
    style css fill:#ccffcc
    subgraph css [Css]
      CssSystem --> Document    
    end

    style query fill:#ccccff
    subgraph query [Query]
      QueryProcessor --> Document
      Query --> QueryProcessor
      Condition --> Query
      SelectType --> Query
    end
    
    style rendering fill:#ffffcc
    subgraph rendering [Rendering]
      Document --> RenderTree
      RenderTree --> Layouter
      RenderTree --> RenderBackend
      Layouter --> RenderBackend
      TreeDrawer --> RenderBackend 
    end  
