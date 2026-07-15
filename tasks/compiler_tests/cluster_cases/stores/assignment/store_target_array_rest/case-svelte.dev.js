import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $a = () => ($.validate_store(a, "a"), $.store_get(a, "$a", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const a = writable(1);
	let rest = $.tag($.mutable_source(), "rest");
	let arr = [
		1,
		2,
		3
	];
	function run() {
		((arr) => {
			var $$array = $.to_array(arr);
			$.store_set(a, $$array[0]);
			$.set(rest, $$array.slice(1));
		})(arr);
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$a() ?? ""}${($.get(rest), $.untrack(() => $.get(rest).length)) ?? ""}`));
	$.event("click", button, run);
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
