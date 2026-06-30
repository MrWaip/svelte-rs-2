import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.snippet(node, () => $$props.snippets.foo);
	$.append($$anchor, fragment);
	$.pop();
}
