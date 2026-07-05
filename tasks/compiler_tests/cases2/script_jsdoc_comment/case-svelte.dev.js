App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[15, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	/** @type {{ name: string, count?: number }} */
	let count = $.prop($$props, "count", 3, 0);
	/** @type {number} */
	let doubled = $.tag($.derived(() => count() * 2), "doubled");
	/** @type {number} */
	let label = $.tag($.derived(() => {
		// format with prefix
		return `${$$props.name}: ${$.get(doubled)}`;
	}), "label");
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(label)));
	$.append($$anchor, p);
	return $.pop($$exports);
}
