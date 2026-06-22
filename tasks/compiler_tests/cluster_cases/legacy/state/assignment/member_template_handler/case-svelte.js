import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>obj</button> <button>arr</button> <p> </p>`, 1);
export default function App($$anchor) {
	let x = $.mutable_source(0);
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	var p = $.sibling(button_1, 2);
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `x: ${$.get(x) ?? ""}`));
	$.event("click", button, () => (($$value) => {
		$.set(x, $$value.x);
		return $$value;
	})({ x: 1 }));
	$.event("click", button_1, () => (($$value) => {
		var $$array = $.to_array($$value, 1);
		$.set(x, $$array[0]);
		return $$value;
	})([2]));
	$.append($$anchor, fragment);
}
