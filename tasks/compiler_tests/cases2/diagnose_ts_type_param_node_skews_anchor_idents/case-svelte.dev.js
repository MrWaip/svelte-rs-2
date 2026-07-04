import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><!></div>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let action;
	let x = $.prop($$props, "x", 8, false);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var node = $.child(div);
	{
		var consequent = ($$anchor) => {
			var text = $.text("a");
			$.append($$anchor, text);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (x()) $$render(consequent);
		}), "if", App, 8, 4);
	}
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
