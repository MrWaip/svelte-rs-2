App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { snippet } from "./stores";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $snippet = () => ($.validate_store(snippet, "snippet"), $.store_get(snippet, "$snippet", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let arg = 1;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.snippet(node, $snippet, () => arg), "render", App, 6, 0);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
