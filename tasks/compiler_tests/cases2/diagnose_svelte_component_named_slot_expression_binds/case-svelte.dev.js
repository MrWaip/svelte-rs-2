import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
var root = $.add_locations($.from_html(`<span slot="caption"></span>`), App[$.FILENAME], [[7, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let current = Inner;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.component(node, () => current, ($$anchor, $$component) => {
		$$component($$anchor, { $$slots: { caption: ($$anchor, $$slotProps) => {
			var span = root();
			span.textContent = "hi";
			$.append($$anchor, span);
		} } });
	}), "component", App, 6, 0, { componentTag: "svelte:component" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
