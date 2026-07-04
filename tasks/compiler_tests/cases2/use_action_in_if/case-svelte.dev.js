App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { tooltip } from "./actions.js";
var root = $.add_locations($.from_html(`<div>text</div>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let show = true;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var div = root();
			$.action(div, ($$node) => tooltip?.($$node));
			$.append($$anchor, div);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (show) $$render(consequent);
		}), "if", App, 6, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
