import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<span> </span>`);
var root_1 = $.from_html(`<button>bump</button> <!>`, 1);
export default function App($$anchor) {
	let state = $.mutable_source("");
	function bump() {
		$.set(state, $.get(state) + "x");
	}
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	{
		var consequent = ($$anchor) => {
			const localLen = $.derived_safe_equal(() => ($.get(state), $.untrack(() => $.get(state).length)));
			var span = root();
			var text = $.child(span);
			$.reset(span);
			$.template_effect(() => $.set_text(text, `Length: ${$.get(localLen) ?? ""}`));
			$.append($$anchor, span);
		};
		$.if(node, ($$render) => {
			if ($.get(state)) $$render(consequent);
		});
	}
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
