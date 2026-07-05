import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
var root_1 = $.from_html(`<button>+</button> <!>`, 1);
export default function App($$anchor) {
	let count = $.mutable_source(1);
	function inc() {
		$.set(count, $.get(count) + 1);
	}
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	{
		var consequent = ($$anchor) => {
			const label = $.derived_safe_equal(() => ($.get(count), $.untrack(() => $.get(count).toFixed(2))));
			var span = root();
			var text = $.child(span, true);
			$.reset(span);
			$.template_effect(() => $.set_text(text, $.get(label)));
			$.append($$anchor, span);
		};
		$.if(node, ($$render) => {
			if ($.get(count)) $$render(consequent);
		});
	}
	$.event("click", button, inc);
	$.append($$anchor, fragment);
}
