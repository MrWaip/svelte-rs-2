App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag($.state(0), "x");
	let z = $.tag($.state(0), "z");
	let arr = [
		1,
		2,
		3
	];
	function update() {
		((arr) => {
			var $$array = $.to_array(arr);
			$.set(x, $$array[0], true);
			$.set(z, $.fallback($$array.slice(1).z, 26), true);
		})(arr);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(x) ?? ""}${$.get(z) ?? ""}`));
	$.delegated("click", button, update);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
