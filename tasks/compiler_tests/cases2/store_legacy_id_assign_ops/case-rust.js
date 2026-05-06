import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { count } from "./stores";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor) {
	const $count = () => $.store_get(count, "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	function go() {
		$.store_set(count, 1);
		$.store_set(count, $count() + 1);
		$.store_set(count, $count() ?? 5);
		$.store_set(count, $count() && 5);
		$.store_set(count, $count() || 5);
		$.update_store(count, $count());
		$.update_store(count, $count(), -1);
		$.update_pre_store(count, $count());
		$.update_pre_store(count, $count(), -1);
	}
	var button = root();
	$.delegated("click", button, go);
	$.append($$anchor, button);
	$$cleanup();
}
$.delegate(["click"]);
