App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { createFormatter } from "./utils.js";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[24, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([]), "items");
	// member on call result: items.filter(Boolean).length
	let total = $.tag($.derived(() => items.filter(Boolean).length), "total");
	// call on import: createFormatter()
	let fmt = createFormatter();
	// member on import: createFormatter.defaults
	let defaults = createFormatter.defaults;
	// member on prop: data.nested
	let nested = $$props.data.nested;
	// new expression
	let map = new Map();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(total) ?? ""} ${fmt ?? ""} ${defaults ?? ""} ${nested ?? ""} ${map ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
