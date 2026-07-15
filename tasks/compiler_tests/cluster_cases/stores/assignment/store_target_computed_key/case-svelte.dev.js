import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $x = () => ($.validate_store(x, "x"), $.store_get(x, "$x", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const x = writable(1);
	const k = "a";
	const obj = { a: 42 };
	function run() {
		$.store_set(x, obj[k]);
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $x()));
	$.event("click", button, run);
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
