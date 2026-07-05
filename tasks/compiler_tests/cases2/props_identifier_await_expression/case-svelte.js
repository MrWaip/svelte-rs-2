import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const props = $.rest_props($$props, rest_excludes);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => fetch(1, 2, 3, $$props.field1), ($$anchor) => {});
	$.append($$anchor, fragment);
	$.pop();
}
