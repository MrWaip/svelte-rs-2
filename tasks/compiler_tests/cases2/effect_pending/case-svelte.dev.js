App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[8, 1]]);
var root_1 = $.add_locations($.from_html(`<p>Done</p>`), App[$.FILENAME], [[10, 1]]);
var root_2 = $.add_locations($.from_html(`<p> </p> <!>`, 1), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	var $$exports = { ...$.legacy_api() };
	var fragment = root_2();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var node = $.sibling(p, 2);
	{
		var consequent = ($$anchor) => {
			var p_1 = root();
			p_1.textContent = "Loading 0";
			$.append($$anchor, p_1);
		};
		var alternate = ($$anchor) => {
			var p_2 = root_1();
			$.append($$anchor, p_2);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.eager($.pending)) $$render(consequent);
			else $$render(alternate, -1);
		}), "if", App, 7, 0);
	}
	$.template_effect(() => $.set_text(text, $.eager($.pending)));
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
