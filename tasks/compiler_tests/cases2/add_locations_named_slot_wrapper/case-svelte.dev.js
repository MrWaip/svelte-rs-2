App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
var root = $.add_locations($.from_html(`<div slot="footer"> </div>`), App[$.FILENAME], [[7, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.prop($$props, "value", 3, "x");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Widget($$anchor, { $$slots: { footer: ($$anchor, $$slotProps) => {
		var div = root();
		var text = $.child(div);
		$.reset(div);
		$.template_effect(() => $.set_text(text, `Footer: ${value() ?? ""}`));
		$.append($$anchor, div);
	} } }), "component", App, 6, 0, { componentTag: "Widget" });
	return $.pop($$exports);
}
