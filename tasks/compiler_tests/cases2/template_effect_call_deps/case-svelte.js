import * as $ from "svelte/internal/client";
import { transform } from "./utils.js";
var root = $.from_html(`<div class="output"> </div>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let value = 0;
	const fn = $.derived(() => transform(value));
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(($0) => $.set_text(text, $0), [() => $.get(fn)(value)]);
	$.append($$anchor, div);
	$.pop();
}
