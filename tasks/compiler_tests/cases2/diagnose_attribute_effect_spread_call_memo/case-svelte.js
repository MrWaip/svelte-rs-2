import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let classes = $.proxy([]);
	function mapClasses(base, ...rest) {
		return { [base]: true };
	}
	var div = root();
	$.attribute_effect(div, ($0) => ({
		...$0,
		[$.CLASS]: { active: true }
	}), [() => mapClasses("base", ...classes)]);
	$.append($$anchor, div);
}
