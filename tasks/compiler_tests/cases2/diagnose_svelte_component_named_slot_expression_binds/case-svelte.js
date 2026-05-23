import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
var root_1 = $.from_html(`<span slot="caption"></span>`);
export default function App($$anchor) {
	let current = Inner;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => current, ($$anchor, $$component) => {
		$$component($$anchor, { $$slots: { caption: ($$anchor, $$slotProps) => {
			var span = root_1();
			span.textContent = "hi";
			$.append($$anchor, span);
		} } });
	});
	$.append($$anchor, fragment);
}
