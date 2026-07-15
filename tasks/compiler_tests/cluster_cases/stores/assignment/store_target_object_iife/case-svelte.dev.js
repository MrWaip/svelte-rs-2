import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $x = () => ($.validate_store(x, "x"), $.store_get(x, "$x", $$stores));
	const $y = () => ($.validate_store(y, "y"), $.store_get(y, "$y", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const x = writable(1);
	const y = writable(2);
	function run() {
		(($$value) => {
			$.store_set(x, $$value.a);
			$.store_set(y, $$value.b);
		})({
			a: 9,
			b: 10
		});
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$x() ?? ""}${$y() ?? ""}`));
	$.event("click", button, run);
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
