import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { count } from "./stores";
var root = $.from_html(`<button>set</button>`);
export default function App($$anchor) {
	const $count = () => $.store_get(count, "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	function set() {
		$.store_set(count, 5);
	}
	var button = root();
	$.delegated("click", button, set);
	$.append($$anchor, button);
	$$cleanup();
}
$.delegate(["click"]);
