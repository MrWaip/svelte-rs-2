import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<button>inc</button> <!>`, 1);
export default function App($$anchor) {
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve(value);
	}
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	{
		var consequent = ($$anchor) => {
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(($0) => $.set_text(text, $0), void 0, [() => delay($.get(x))]);
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if ($.get(x) >= 0) $$render(consequent);
		});
	}
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
