import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.tag($.mutable_source(0), "a");
	let arr = [1];
	function update() {
		let x = ((arr) => {
			var $$array = $.to_array(arr, 1);
			$.set(a, $$array[0]);
			return arr;
		})(arr);
		return x;
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
