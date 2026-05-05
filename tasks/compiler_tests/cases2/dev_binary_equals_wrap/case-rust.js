App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<!> `, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = 1;
	let b = 2;
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("equal");
			$.append($$anchor, text);
		};
		var consequent_1 = ($$anchor) => {
			var text_1 = $.text("one");
			$.append($$anchor, text_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.strict_equals(a, b)) $$render(consequent);
			else if ($.equals(a, 1)) $$render(consequent_1, 1);
		}), "if", App, 6, 0);
	}
	var text_2 = $.sibling(node);
	text_2.nodeValue = ` ${$.strict_equals(a, b, false) ?? ""}
${$.equals(a, 1) ?? ""}
${$.equals(a, 0, false) ?? ""}`;
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
