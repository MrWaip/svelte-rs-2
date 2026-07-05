import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.tag($.mutable_source(0), "a");
	let arr = [[1]];
	function update() {
		((arr) => {
			var $$array = $.to_array(arr, 1);
			var $$array_1 = $.to_array($$array[0], 1);
			$.set(a, $$array_1[0]);
		})(arr);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(a)));
	$.event("click", button, update);
	$.append($$anchor, button);
	return $.pop($$exports);
}
