import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $s = () => $.store_get(s, "$s", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const s = writable(0);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.snippet(node, () => $$props.children, $s);
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
