import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <!>`, 1);
export default function App($$anchor) {
	let x = $.state(0);
	let go = false;
	function delay(value) {
		return Promise.resolve(value);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.async(node_1, void 0, [() => delay($.get(x))], ($$anchor, $0) => {
				App(node_1, { get value() {
					return $.get($0);
				} });
			});
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (go) $$render(consequent);
		});
	}
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
