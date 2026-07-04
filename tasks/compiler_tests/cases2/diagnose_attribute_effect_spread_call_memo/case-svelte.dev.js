App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let classes = $.tag_proxy($.proxy([]), "classes");
	function mapClasses(base, ...rest) {
		return { [base]: true };
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, ($0) => ({
		...$0,
		[$.CLASS]: { active: true }
	}), [() => mapClasses("base", ...classes)]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
