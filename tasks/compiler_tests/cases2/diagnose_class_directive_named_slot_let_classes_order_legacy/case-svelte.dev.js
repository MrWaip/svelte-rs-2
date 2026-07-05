import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div slot="activator">hi</div>`), App[$.FILENAME], [[8, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let active = $.prop($$props, "active", 8);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Outer($$anchor, { $$slots: { activator: ($$anchor, $$slotProps) => {
		var div = root();
		let classes;
		$.template_effect(() => classes = $.set_class(div, 1, "", null, classes, { active: active() }));
		$.append($$anchor, div);
	} } }), "component", App, 7, 0, { componentTag: "Outer" });
	return $.pop($$exports);
}
