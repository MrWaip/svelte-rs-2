App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { slide } from "svelte/transition";
var root = $.add_locations($.from_html(`<div><button>hi</button></div>`), App[$.FILENAME], [[
	8,
	1,
	[[9, 2]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let visible = true;
	function k() {}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var div = root();
			var button = $.child(div);
			$.reset(div);
			$.delegated("click", button, k);
			$.transition(3, div, () => slide);
			$.append($$anchor, div);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (visible) $$render(consequent);
		}), "if", App, 7, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
