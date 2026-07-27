import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	async function g() {
		return 2;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.element(node, () => $$props.tag, false, ($$element, $$anchor) => {
		$.attribute_effect($$element, ($0) => ({ title: $0 }), void 0, [() => g()]);
	});
	$.append($$anchor, fragment);
}
