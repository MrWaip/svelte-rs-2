import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let x = $.tag($.mutable_source(0), "x");
	let z = $.tag($.mutable_source(0), "z");
	let arr = [
		1,
		2,
		3
	];
	function update() {
		((arr) => {
			var $$array = $.to_array(arr);
			$.set(x, $$array[0]);
			$.set(z, $.fallback($$array.slice(1).z, 26));
		})(arr);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""}${$.get(z) ?? ""}`));
	$.event("click", button, update);
	$.append($$anchor, button);
	return $.pop($$exports);
}
