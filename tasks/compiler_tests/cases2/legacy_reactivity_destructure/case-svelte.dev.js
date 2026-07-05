import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let point = {
		left: 1,
		right: 2
	};
	let tmp = point, left = $.tag($.mutable_source(tmp.left), "left"), right = $.tag($.mutable_source(tmp.right), "right");
	function swap() {
		(($$value) => {
			var $$array = $.to_array($$value, 2);
			$.set(left, $$array[0]);
			$.set(right, $$array[1]);
		})([$.get(right), $.get(left)]);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(left) ?? ""}:${$.get(right) ?? ""}`));
	$.delegated("click", button, swap);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
