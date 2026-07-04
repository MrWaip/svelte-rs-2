App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let id = $.prop($$props, "id", 3, "");
	function getStyle() {
		return "color:red;";
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.template_effect(($0) => {
		$.set_attribute(div, "data-testid", id());
		$.set_style(div, $0);
	}, [() => getStyle()]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
