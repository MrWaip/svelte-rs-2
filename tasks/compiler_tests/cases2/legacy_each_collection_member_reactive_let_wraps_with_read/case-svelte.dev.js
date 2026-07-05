import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[14, 4]]);
var root_1 = $.add_locations($.from_html(`<button>swap</button> <!>`, 1), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const filters = [{ data: [1] }, { data: [2] }];
	let modeData = $.tag($.mutable_source(filters[0]), "modeData");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.add_svelte_meta(() => $.each(node, 1, () => ($.get(modeData), $.untrack(() => $.get(modeData).data)), (curtain) => curtain, ($$anchor, curtain) => {
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(curtain)));
		$.append($$anchor, div);
	}), "each", App, 13, 0);
	$.delegated("click", button, function click() {
		$.set(modeData, filters[1]);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
