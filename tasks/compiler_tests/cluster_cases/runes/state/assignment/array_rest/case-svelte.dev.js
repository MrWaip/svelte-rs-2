App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = $.tag($.state(0), "a");
	let b = $.tag($.state(0), "b");
	let arr = [
		1,
		2,
		3
	];
	function update() {
		((arr) => {
			var $$array = $.to_array(arr);
			$.set(a, $$array[0], true);
			$.set(b, $$array.slice(1), true);
		})(arr);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.delegated("click", button, update);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
