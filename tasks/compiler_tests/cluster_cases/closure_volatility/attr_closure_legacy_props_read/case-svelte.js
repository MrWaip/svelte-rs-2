import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div> <button> </button>`, 1);
export default function App($$anchor, $$props) {
	const $$sanitized_props = $.legacy_rest_props($$props, [
		"children",
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	$.push($$props, false);
	let a = $.prop($$props, "a", 8);
	let count = $.mutable_source(0);
	function bump() {
		$.set(count, $.get(count) + 1);
	}
	$.init();
	var fragment = root();
	var div = $.first_child(fragment);
	$.set_attribute(div, "title", [() => $$sanitized_props.x]);
	var text = $.child(div, true);
	$.reset(div);
	var button = $.sibling(div, 2);
	var text_1 = $.child(button, true);
	$.reset(button);
	$.template_effect(() => {
		$.set_text(text, a());
		$.set_text(text_1, $.get(count));
	});
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
	$.pop();
}
$.delegate(["click"]);
