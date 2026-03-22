import * as $ from "svelte/internal/client";
import { createFormatter } from "./utils.js";
var root = $.from_html(`<p></p>`);
export default function App($$anchor, $$props) {
	let items = $.proxy([]);
	// member on call result: items.filter(Boolean).length
	let total = $.derived(() => items.filter(Boolean).length);
	// call on import: createFormatter()
	let fmt = createFormatter();
	// member on import: createFormatter.defaults
	let defaults = createFormatter.defaults;
	// member on prop: data.nested
	let nested = $$props.data.nested;
	// new expression
	let map = new Map();
	var p = root();
	p.textContent = `${$.get(total) ?? ""} ${fmt ?? ""} ${defaults ?? ""} ${nested ?? ""} ${map ?? ""}`;
	$.append($$anchor, p);
}
