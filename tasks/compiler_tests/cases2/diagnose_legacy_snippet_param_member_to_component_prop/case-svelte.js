import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, items, $.index, ($$anchor, item) => {
		const row = ($$anchor, $$arg0) => {
			let value = () => $$arg0?.().value;
			Child($$anchor, { get name() {
				return value(), $.untrack(() => value().name);
			} });
		};
		row($$anchor, () => ({ value: $.get(item) }));
	});
	$.append($$anchor, fragment);
}
