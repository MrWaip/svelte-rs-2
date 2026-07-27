import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
const row = ($$anchor, value = $.noop) => {
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, value()));
	$.append($$anchor, p);
};
var root = $.from_html(`<p> </p>`);
var root_1 = $.from_html(`<button>inc</button> <!>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve({ value });
	}
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, void 0, [async () => await (await $.save(delay($.get(x))))().value], (node, $0) => {
		row(node, () => $.get($0));
	});
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
	$.pop();
}
$.delegate(["click"]);
