import * as $ from "svelte/internal/client";
const recurse = ($$anchor) => {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	App(node, {});
	$.append($$anchor, fragment);
};
export default function App($$anchor) {
	recurse($$anchor);
}
