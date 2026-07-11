import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $s = () => ($.validate_store(s, "s"), $.store_get(s, "$s", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const s = writable(1);
	let plain = $.tag($.mutable_source(), "plain");
	function run() {
		(($$value) => {
			var $$array = $.to_array($$value, 2);
			$.set(plain, $$array[0]);
			$.store_set(s, $$array[1]);
		})([1, 2]);
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(plain) ?? ""}${$s() ?? ""}`));
	$.event("click", button, run);
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
