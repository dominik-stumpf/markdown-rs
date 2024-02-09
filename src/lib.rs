#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]

// extern crate markdown;
mod hast;
mod hast_util_to_swc;
mod mdast_util_to_hast;
mod mdx_plugin_recma_document;
mod mdx_plugin_recma_jsx_rewrite;
mod swc;
mod swc_util_build_jsx;
mod swc_utils;

use crate::{
    hast_util_to_swc::hast_util_to_swc,
    mdast_util_to_hast::mdast_util_to_hast,
    mdx_plugin_recma_document::{mdx_plugin_recma_document, Options as DocumentOptions},
    mdx_plugin_recma_jsx_rewrite::{mdx_plugin_recma_jsx_rewrite, Options as RewriteOptions},
    swc::{parse_esm, parse_expression, serialize},
    swc_util_build_jsx::{swc_util_build_jsx, Options as BuildOptions},
};

// use crate::{to_mdast, Constructs, Location, ParseOptions};

pub use crate::configuration::{MdxConstructs, MdxParseOptions};
pub use crate::mdx_plugin_recma_document::JsxRuntime;

/// Turn MDX into JavaScript.
///
/// ## Examples
///
/// ```
/// use mdxjs::compile;
/// # fn main() -> Result<(), String> {
///
/// assert_eq!(compile("# Hi!", &Default::default())?, "import { jsx as _jsx } from \"react/jsx-runtime\";\nfunction _createMdxContent(props) {\n    const _components = Object.assign({\n        h1: \"h1\"\n    }, props.components);\n    return _jsx(_components.h1, {\n        children: \"Hi!\"\n    });\n}\nfunction MDXContent(props = {}) {\n    const { wrapper: MDXLayout } = props.components || {};\n    return MDXLayout ? _jsx(MDXLayout, Object.assign({}, props, {\n        children: _jsx(_createMdxContent, props)\n    })) : _createMdxContent(props);\n}\nexport default MDXContent;\n");
/// # Ok(())
/// # }
/// ```
///
/// ## Errors
///
/// This project errors for many different reasons, such as syntax errors in
/// the MDX format or misconfiguration.
// pub fn compile(value: &str, options: &Options) -> Result<String, String> {
//     let parse_options = ParseOptions {
//         constructs: Constructs {
//             attention: options.parse.constructs.attention,
//             autolink: false,
//             block_quote: options.parse.constructs.block_quote,
//             character_escape: options.parse.constructs.character_escape,
//             character_reference: options.parse.constructs.character_reference,
//             code_fenced: options.parse.constructs.code_fenced,
//             code_indented: false,
//             code_text: options.parse.constructs.code_text,
//             definition: options.parse.constructs.definition,
//             frontmatter: options.parse.constructs.frontmatter,
//             gfm_autolink_literal: options.parse.constructs.gfm_autolink_literal,
//             gfm_footnote_definition: options.parse.constructs.gfm_footnote_definition,
//             gfm_label_start_footnote: options.parse.constructs.gfm_label_start_footnote,
//             gfm_strikethrough: options.parse.constructs.gfm_strikethrough,
//             gfm_table: options.parse.constructs.gfm_table,
//             gfm_task_list_item: options.parse.constructs.gfm_task_list_item,
//             hard_break_escape: options.parse.constructs.hard_break_escape,
//             hard_break_trailing: options.parse.constructs.hard_break_trailing,
//             html_flow: false,
//             html_text: false,
//             heading_atx: options.parse.constructs.heading_atx,
//             heading_setext: options.parse.constructs.heading_setext,
//             label_start_image: options.parse.constructs.label_start_image,
//             label_start_link: options.parse.constructs.label_start_link,
//             label_end: options.parse.constructs.label_end,
//             list_item: options.parse.constructs.list_item,
//             math_flow: options.parse.constructs.math_flow,
//             math_text: options.parse.constructs.math_text,
//             mdx_esm: true,
//             mdx_expression_flow: true,
//             mdx_expression_text: true,
//             mdx_jsx_flow: true,
//             mdx_jsx_text: true,
//             thematic_break: options.parse.constructs.thematic_break,
//         },
//         gfm_strikethrough_single_tilde: options.parse.gfm_strikethrough_single_tilde,
//         math_text_single_dollar: options.parse.math_text_single_dollar,
//         mdx_esm_parse: Some(Box::new(parse_esm)),
//         mdx_expression_parse: Some(Box::new(parse_expression)),
//     };
//     let document_options = DocumentOptions {
//         pragma: options.pragma.clone(),
//         pragma_frag: options.pragma_frag.clone(),
//         pragma_import_source: options.pragma_import_source.clone(),
//         jsx_import_source: options.jsx_import_source.clone(),
//         jsx_runtime: options.jsx_runtime,
//     };
//     let rewrite_options = RewriteOptions {
//         development: options.development,
//         provider_import_source: options.provider_import_source.clone(),
//     };
//     let build_options = BuildOptions {
//         development: options.development,
//     };
//
//     let location = Location::new(value.as_bytes());
//     let mdast = to_mdast(value, &parse_options)?;
//     let hast = mdast_util_to_hast(&mdast);
//     let mut program = hast_util_to_swc(&hast, options.filepath.clone(), Some(&location))?;
//     mdx_plugin_recma_document(&mut program, &document_options, Some(&location))?;
//     mdx_plugin_recma_jsx_rewrite(&mut program, &rewrite_options, Some(&location));
//
//     if !options.jsx {
//         swc_util_build_jsx(&mut program, &build_options, Some(&location))?;
//     }
//
//     Ok(serialize(&mut program.module, Some(&program.comments)))
// }
extern crate alloc;
mod configuration;
mod construct;
mod event;
mod parser;
mod resolve;
mod state;
mod subtokenize;
mod to_html;
mod to_mdast;
mod tokenizer;
mod util;

pub mod mdast; // To do: externalize?
pub mod unist; // To do: externalize.

pub fn md_to_hast(value: &str) -> hast::Node {
    let mdast = to_mdast(value, &ParseOptions::default()).unwrap();
    let hast = mdast_util_to_hast(&mdast);

    return hast
}

#[doc(hidden)]
pub use util::identifier::{id_cont, id_start};

#[doc(hidden)]
pub use util::sanitize_uri::sanitize;

#[doc(hidden)]
pub use util::location::Location;

pub use util::line_ending::LineEnding;

pub use util::mdx::{
    EsmParse as MdxEsmParse, ExpressionKind as MdxExpressionKind,
    ExpressionParse as MdxExpressionParse, Signal as MdxSignal,
};

pub use configuration::{CompileOptions, Constructs, Options, ParseOptions};

use alloc::string::String;

/// Turn markdown into HTML.
///
/// Compiles markdown to HTML according to `CommonMark`.
/// Use [`to_html_with_options()`][] to configure how markdown is turned into
/// HTML.
///
/// ## Examples
///
/// ```
/// use markdown::to_html;
///
/// assert_eq!(to_html("# Hello, world!"), "<h1>Hello, world!</h1>");
/// ```
pub fn to_html(value: &str) -> String {
    to_html_with_options(value, &Options::default()).unwrap()
}

/// Turn markdown into HTML, with configuration.
///
/// ## Errors
///
/// `to_html_with_options()` never errors with normal markdown because markdown
/// does not have syntax errors, so feel free to `unwrap()`.
/// However, MDX does have syntax errors.
/// When MDX is turned on, there are several errors that can occur with how
/// expressions, ESM, and JSX are written.
///
/// ## Examples
///
/// ```
/// use markdown::{to_html_with_options, CompileOptions, Options};
/// # fn main() -> Result<(), String> {
///
/// // Use GFM:
/// let result = to_html_with_options("~hi~hello!", &Options::gfm())?;
///
/// assert_eq!(result, "<p><del>hi</del>hello!</p>");
///
/// // Live dangerously / trust the author:
/// let result = to_html_with_options("<div>\n\n# Hello, world!\n\n</div>", &Options {
///     compile: CompileOptions {
///       allow_dangerous_html: true,
///       allow_dangerous_protocol: true,
///       ..CompileOptions::default()
///     },
///     ..Options::default()
/// })?;
///
/// assert_eq!(result, "<div>\n<h1>Hello, world!</h1>\n</div>");
/// # Ok(())
/// # }
/// ```
pub fn to_html_with_options(value: &str, options: &Options) -> Result<String, String> {
    let (events, parse_state) = parser::parse(value, &options.parse)?;
    Ok(to_html::compile(
        &events,
        parse_state.bytes,
        &options.compile,
    ))
}

/// Turn markdown into a syntax tree.
///
/// ## Errors
///
/// `to_mdast()` never errors with normal markdown because markdown does not
/// have syntax errors, so feel free to `unwrap()`.
/// However, MDX does have syntax errors.
/// When MDX is turned on, there are several errors that can occur with how
/// JSX, expressions, or ESM are written.
///
/// ## Examples
///
/// ```
/// use markdown::{to_mdast, ParseOptions};
/// # fn main() -> Result<(), String> {
///
/// let tree = to_mdast("# Hey, *you*!", &ParseOptions::default())?;
///
/// println!("{:?}", tree);
/// // => Root { children: [Heading { children: [Text { value: "Hey, ", position: Some(1:3-1:8 (2-7)) }, Emphasis { children: [Text { value: "you", position: Some(1:9-1:12 (8-11)) }], position: Some(1:8-1:13 (7-12)) }, Text { value: "!", position: Some(1:13-1:14 (12-13)) }], position: Some(1:1-1:14 (0-13)), depth: 1 }], position: Some(1:1-1:14 (0-13)) }
/// # Ok(())
/// # }
/// ```
pub fn to_mdast(value: &str, options: &ParseOptions) -> Result<mdast::Node, String> {
    let (events, parse_state) = parser::parse(value, options)?;
    let node = to_mdast::compile(&events, parse_state.bytes)?;
    Ok(node)
}
