import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="action svelte-1nvkc8o">fallback</div>`), App[$.FILENAME], [[3, 8]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Wrap($$anchor, { $$slots: { action: ($$anchor, $$slotProps) => {
		var fragment_1 = $.comment();
		var node = $.first_child(fragment_1);
		$.slot(node, $$props, "action", {}, ($$anchor) => {
			var div = root();
			$.append($$anchor, div);
		});
		$.append($$anchor, fragment_1);
	} } }), "component", App, 1, 0, { componentTag: "Wrap" });
	return $.pop($$exports);
}
