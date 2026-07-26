import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>yes</p>`);
var root_1 = $.from_html(`<!> <button>inc</button>`, 1);
export default function App($$anchor) {
	let count = 0;
	async function check(v) {
		return v > 0;
	}
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.async(node, [], [() => check(count)], (node, $$condition) => {
		var consequent = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if ($.get($$condition)) $$render(consequent);
		});
	});
	var button = $.sibling(node, 2);
	$.delegated("click", button, () => count++);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
