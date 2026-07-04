App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = $.tag($.state(0), "a");
	let obj = { x: 0 };
	let arr = [1, 2];
	function update() {
		((arr) => {
			var $$array = $.to_array(arr, 2);
			obj.x = $$array[0];
			$.set(a, $$array[1], true);
		})(arr);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(a)));
	$.delegated("click", button, update);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
