import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { count } from "./stores";
var root = $.add_locations($.from_html(`<button>set</button>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $count = () => ($.validate_store(count, "count"), $.store_get(count, "$count", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	function set() {
		$.store_set(count, 5);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, set);
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
