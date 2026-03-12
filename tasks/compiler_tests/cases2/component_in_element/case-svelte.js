import * as $ from "svelte/internal/client";
import Button from "./Button.svelte";
var root = $.from_html(`<div><!></div>`);
export default function App($$anchor) {
	var div = root();
	var node = $.child(div);
	Button(node, {});
	$.reset(div);
	$.append($$anchor, div);
}
