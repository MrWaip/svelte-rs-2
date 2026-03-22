import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	/** @type {{ name: string, count?: number }} */
	let count = $.prop($$props, "count", 3, 0);
	/** @type {number} */
	let doubled = $.derived(() => count() * 2);
	/** @type {number} */
	let label = $.derived(() => {
		// format with prefix
		return `${$$props.name}: ${$.get(doubled)}`;
	});
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(label)));
	$.append($$anchor, p);
}
