import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<!> <button>inc</button>`, 1);
export default function App($$anchor) {
	const row = ($$anchor) => {
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(($0) => $.set_text(text, $0), void 0, [() => compute(count)]);
		$.append($$anchor, p);
	};
	let count = 0;
	async function compute(v) {
		return v * 2;
	}
	var fragment = root_1();
	var node = $.first_child(fragment);
	row(node);
	var button = $.sibling(node, 2);
	$.delegated("click", button, () => count++);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
