import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let point = {
		left: 1,
		right: 2
	};
	let tmp = point, left = $.mutable_source(tmp.left), right = $.mutable_source(tmp.right);
	function swap() {
		(($$value) => {
			var $$array = $.to_array($$value, 2);
			$.set(left, $$array[0]);
			$.set(right, $$array[1]);
		})([$.get(right), $.get(left)]);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(left) ?? ""}:${$.get(right) ?? ""}`));
	$.delegated("click", button, swap);
	$.append($$anchor, button);
}
$.delegate(["click"]);
