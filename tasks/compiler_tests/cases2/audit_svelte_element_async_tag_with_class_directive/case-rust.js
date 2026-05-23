import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let active = false;
	async function getTag() {
		return "div";
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [getTag], (node, $$tag) => {
		$.element(node, () => $.get($$tag), false, ($$element, $$anchor) => {
			$.set_class($$element, 0, "", null, {}, { active });
			var text = $.text("x");
			$.append($$anchor, text);
		});
	});
	$.append($$anchor, fragment);
}
