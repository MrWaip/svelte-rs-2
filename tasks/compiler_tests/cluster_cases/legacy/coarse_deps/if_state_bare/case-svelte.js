import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>+</button><!>`, 1);
export default function App($$anchor) {
	let foo = $.mutable_source(1);
	function inc() {
		$.set(foo, $.get(foo) + 1);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button);
	{
		var consequent = ($$anchor) => {
			var text = $.text("a");
			$.append($$anchor, text);
		};
		$.if(node, ($$render) => {
			if ($.get(foo)) $$render(consequent);
		});
	}
	$.event("click", button, inc);
	$.append($$anchor, fragment);
}
