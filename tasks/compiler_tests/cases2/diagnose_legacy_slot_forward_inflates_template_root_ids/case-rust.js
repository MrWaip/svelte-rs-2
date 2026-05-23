import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<div class="action svelte-1nvkc8o">fallback</div>`);
export default function App($$anchor, $$props) {
	Wrap($$anchor, { $$slots: { action: ($$anchor, $$slotProps) => {
		var fragment_1 = $.comment();
		var node = $.first_child(fragment_1);
		$.slot(node, $$props, "action", {}, ($$anchor) => {
			var div = root_2();
			$.append($$anchor, div);
		});
		$.append($$anchor, fragment_1);
	} } });
}
