mod code_block;
mod list_item;

use std::path::Path;

use leptos::html::AnyElement;
use leptos::*;
use markdown::mdast::Node;

use self::code_block::CodeBlock;
use self::list_item::ListItem;
use crate::backend::convert_file_url;
use crate::backend::file::open;

pub fn render_md_node(node: &Node) -> HtmlElement<AnyElement> {
    match node {
        Node::Root(root) => view! {
            <div class="note">
                {root.children
                    .iter()
                    .map(render_md_node)
                    .collect::<Vec<_>>()}
            </div>
        }
        .into_any(),
        Node::Paragraph(paragraph) => view! {
            <span class="paragraph">
                {paragraph.children
                    .iter()
                    .map(render_md_node)
                    .collect::<Vec<_>>()}
            </span>
        }
        .into_any(),
        Node::ThematicBreak(_) => view! { <br class="br" /> }.into_any(),
        Node::Heading(heading) => {
            let inner = heading.children.iter().map(render_md_node).collect::<Vec<_>>();
            match heading.depth {
                1 => view! {
                    <h1>{inner}</h1>
                }
                .into_any(),
                2 => view! {
                    <h2>{inner}</h2>
                }
                .into_any(),
                3 => view! {
                    <h3>{inner}</h3>
                }
                .into_any(),
                4 => view! {
                    <h4>{inner}</h4>
                }
                .into_any(),
                5 => view! {
                    <h5>{inner}</h5>
                }
                .into_any(),
                6 => view! {
                    <h6>{inner}</h6>
                }
                .into_any(),
                // Note: should never be reached but let it be here just in case.
                n => view! {
                    <span>{format!("Heading with depth={} is not supported!", n)}</span>
                }
                .into_any(),
            }
        }
        Node::InlineCode(code) => {
            let code_value = code.value.clone();
            view! {
                <span class="incline-code" on:click=move |_| {
                    let clipboard = window().navigator().clipboard();
                    let _ = clipboard.write_text(&code_value);
                }>{&code.value}</span>
            }
            .into_any()
        }
        Node::Text(text) => view! { <span class="text">{&text.value}</span> }.into_any(),
        Node::Delete(delete) => view! {
            <s>
                {delete.children
                    .iter()
                    .map(render_md_node)
                    .collect::<Vec<_>>()}
            </s>
        }
        .into_any(),
        Node::Emphasis(emphasis) => view! {
            <em>
                {emphasis.children
                    .iter()
                    .map(render_md_node)
                    .collect::<Vec<_>>()}
            </em>
        }
        .into_any(),
        Node::Strong(strong) => view! {
            <b>
                {strong.children
                    .iter()
                    .map(render_md_node)
                    .collect::<Vec<_>>()}
            </b>
        }
        .into_any(),
        Node::Blockquote(quote) => view! {
            <div class="quote">
                {quote.children
                    .iter()
                    .map(render_md_node)
                    .collect::<Vec<_>>()}
            </div>
        }
        .into_any(),
        Node::Link(link) => view! {
            <a class="link" href=&link.url target="popup">
                {link.children
                    .iter()
                    .map(render_md_node)
                    .collect::<Vec<_>>()}
            </a>
        }
        .into_any(),
        Node::List(list) => {
            if list.ordered {
                view! {
                    <ol class="list" start=list.start.unwrap_or(1)>
                        {list.children
                            .iter()
                            .map(render_md_node)
                            .collect::<Vec<_>>()}
                    </ol>
                }
                .into_any()
            } else {
                view! {
                    <ul class="list">
                        {list.children
                            .iter()
                            .map(render_md_node)
                            .collect::<Vec<_>>()}
                    </ul>
                }
                .into_any()
            }
        }
        Node::ListItem(list_item) => view! {
            <div>
                <ListItem list_item=list_item.clone() />
            </div>
        }
        .into_any(),
        Node::Image(image) => {
            let image_path = image.url.clone();
            let open_image = move |_| {
                let path = image_path.clone();
                spawn_local(async move {
                    open(Path::new(&convert_file_url(path))).await;
                })
            };
            view! {
                <img src=image.url.clone() alt=image.alt.clone() class="note-image" on:click=open_image />
            }
        }
        .into_any(),
        Node::Table(table) => view! {
            <table class="table">
                {
                    if let Node::TableRow(header_row) = table.children.first().unwrap() {
                        view! {
                            <tr class="table-header">
                                {header_row.children
                                    .iter()
                                    .map(render_md_node)
                                    .collect_view()}
                            </tr>
                        }
                        .into_any()
                    } else {
                        view! {
                            <span>"The first Table children should be TableRow"</span>
                        }
                        .into_any()
                    }
                }
                {
                    table.children[1..]
                        .iter()
                        .map(render_md_node)
                        .collect_view()
                }
            </table>
        }
        .into_any(),
        Node::TableRow(row) => view! {
            <tr class="table-row">
                {row.children
                    .iter()
                    .map(render_md_node)
                    .collect_view()}
            </tr>
        }
        .into_any(),
        Node::TableCell(cell) => view! {
            <td class="table-cell">
                {cell.children
                    .iter()
                    .map(render_md_node)
                    .collect_view()}
            </td>
        }
        .into_any(),
        Node::Code(code) => {
            let lang = code.lang.clone().unwrap_or_else(|| String::from("txt"));
            view! {
                <div>
                    <CodeBlock code=code.value.clone() lang/>
                </div>
            }
            .into_any()
        }
        v => view! { <span>{format!("{:?} is not supported", v)}</span> }.into_any(),
    }
}