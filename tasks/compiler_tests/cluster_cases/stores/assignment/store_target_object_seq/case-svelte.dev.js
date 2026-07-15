import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $a = () => ($.validate_store(a, "a"), $.store_get(a, "$a", $$stores));
	const $b = () => ($.validate_store(b, "b"), $.store_get(b, "$b", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const a = writable(1);
	const b = writable(2);
	const obj = {
		$a: 10,
		$b: 20
	};
	function run() {
		$.store_set(a, obj.$a), $.store_set(b, obj.$b);
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$a() ?? ""}${$b() ?? ""}`));
	$.event("click", button, run);
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
