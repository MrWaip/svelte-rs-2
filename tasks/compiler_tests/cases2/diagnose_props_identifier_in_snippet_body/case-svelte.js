import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
export default function App($$anchor, $$props) {
	const foo = ($$anchor) => {
		var fragment = $.comment();
		var node = $.first_child(fragment);
		$.component(node, () => $$props.X, ($$anchor, props_X) => {
			props_X($$anchor, {});
		});
		$.append($$anchor, fragment);
	};
	let props = $.rest_props($$props, rest_excludes);
}
