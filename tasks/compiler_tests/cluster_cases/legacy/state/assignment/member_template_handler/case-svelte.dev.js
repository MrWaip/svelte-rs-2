import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>obj</button> <button>arr</button> <p> </p>`, 1), App[$.FILENAME], [
	[4, 0],
	[5, 0],
	[6, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let x = $.tag($.mutable_source(0), "x");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	var p = $.sibling(button_1, 2);
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `x: ${$.get(x) ?? ""}`));
	$.event("click", button, function click() {
		return (($$value) => {
			$.set(x, $$value.x);
			return $$value;
		})({ x: 1 });
	});
	$.event("click", button_1, function click_1() {
		return (($$value) => {
			var $$array = $.to_array($$value, 1);
			$.set(x, $$array[0]);
			return $$value;
		})([2]);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
