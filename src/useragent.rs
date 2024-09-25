fn main() {
    main_do_things::<MyModuleConfiguration>();
}

fn main_do_things<C: ModuleConfiguration>() {
    let backend = C::RenderBackend::new();
    let css_system = C::CssSystem::new();
    let document = C::Document::new();
    let html_parser = C::HtmlParser::new();
    let layouter = C::Layouter::new();
    let tree_drawer = C::TreeDrawer::new();
    let render_tree = C::RenderTree::new();

    css_system.do_css_things();
    document.do_document_things();
    html_parser.do_html_parser_things();
    layouter.do_layouter_things();
    tree_drawer.do_tree_drawer_things();
    render_tree.do_render_tree_things();
    backend.do_render_backend_things();
}