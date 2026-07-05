import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let component;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.component(node, () => component, ($$anchor, $$component) => {
		$$component($$anchor, { $$slots: { "empty-state": ($$anchor, $$slotProps) => {
			var text = $.text("empty");
			$.append($$anchor, text);
		} } });
	}), "component", App, 7, 0, { componentTag: "svelte:component" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
