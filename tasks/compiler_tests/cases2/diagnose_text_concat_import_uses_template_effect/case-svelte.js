import * as $ from "svelte/internal/client";
import { NAME } from "./lib";
import Other from "./Other.svelte";
var root = $.from_html(`<div> <!></div>`);
export default function App($$anchor) {
	var div = root();
	var text = $.child(div);
	var node = $.sibling(text);
	Other(node, {});
	$.reset(div);
	$.template_effect(() => $.set_text(text, `Hello ${NAME ?? ""} world`));
	$.append($$anchor, div);
}
