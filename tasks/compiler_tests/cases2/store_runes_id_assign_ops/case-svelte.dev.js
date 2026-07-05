App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { count } from "./stores";
var root = $.add_locations($.from_html(`<button>go</button>`), App[$.FILENAME], [[26, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $count = () => ($.validate_store(count, "count"), $.store_get(count, "$count", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	function go() {
		$.store_set(count, 1);
		$.store_set(count, $count() + 1);
		$.store_set(count, $count() - 1);
		$.store_set(count, $count() * 2);
		$.store_set(count, $count() / 2);
		$.store_set(count, $count() % 3);
		$.store_set(count, $count() ** 2);
		$.store_set(count, $count() && 5);
		$.store_set(count, $count() || 5);
		$.store_set(count, $count() ?? 5);
		$.store_set(count, $count() & 1);
		$.store_set(count, $count() | 1);
		$.store_set(count, $count() ^ 1);
		$.store_set(count, $count() << 1);
		$.store_set(count, $count() >> 1);
		$.store_set(count, $count() >>> 1);
		$.update_store(count, $count());
		$.update_store(count, $count(), -1);
		$.update_pre_store(count, $count());
		$.update_pre_store(count, $count(), -1);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, go);
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
