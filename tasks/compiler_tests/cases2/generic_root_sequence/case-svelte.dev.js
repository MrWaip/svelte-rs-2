App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`some text <div></div> <input/> <div></div> <!>`, 1), App[$.FILENAME], [
	[2, 0],
	[4, 0],
	[6, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var fragment = root();
	var text = $.sibling($.first_child(fragment), 2);
	text.nodeValue = ` ${some_variable ?? ""} `;
	var text_1 = $.sibling(text, 2);
	text_1.nodeValue = ` text + ${name ?? ""} `;
	var node = $.sibling(text_1, 3);
	{
		var consequent = ($$anchor) => {};
		var consequent_1 = ($$anchor) => {};
		var alternate = ($$anchor) => {};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (true) $$render(consequent);
			else if (false) $$render(consequent_1, 1);
			else $$render(alternate, -1);
		}), "if", App, 7, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
