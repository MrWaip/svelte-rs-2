import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let component;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => component, ($$anchor, $$component) => {
		$$component($$anchor, { $$slots: { "empty-state": ($$anchor, $$slotProps) => {
			var text = $.text("empty");
			$.append($$anchor, text);
		} } });
	});
	$.append($$anchor, fragment);
}
