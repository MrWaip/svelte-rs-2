import * as $ from "svelte/internal/client";
import { count } from "./stores";
var root = $.from_html(`<button>set</button>`);
export default function App($$anchor) {
	const $count = () => $.store_get(count, "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var button = root();
	$.delegated("click", button, () => $.store_set(count, 5));
	$.append($$anchor, button);
	$$cleanup();
}
$.delegate(["click"]);
