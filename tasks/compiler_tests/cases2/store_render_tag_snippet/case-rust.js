import * as $ from "svelte/internal/client";
import { snippet } from "./stores";
export default function App($$anchor) {
	const $snippet = () => $.store_get(snippet, "$snippet", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let arg = 1;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.snippet(node, $snippet, () => arg);
	$.append($$anchor, fragment);
	$$cleanup();
}
