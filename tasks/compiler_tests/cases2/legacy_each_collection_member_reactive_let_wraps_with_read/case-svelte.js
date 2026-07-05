import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
var root_1 = $.from_html(`<button>swap</button> <!>`, 1);
export default function App($$anchor) {
	const filters = [{ data: [1] }, { data: [2] }];
	let modeData = $.mutable_source(filters[0]);
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.each(node, 1, () => ($.get(modeData), $.untrack(() => $.get(modeData).data)), (curtain) => curtain, ($$anchor, curtain) => {
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(curtain)));
		$.append($$anchor, div);
	});
	$.delegated("click", button, () => {
		$.set(modeData, filters[1]);
	});
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
