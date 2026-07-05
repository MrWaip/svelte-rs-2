App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<meta name="a" content="b"/>`), App[$.FILENAME], [[10, 8]]);
var root_1 = $.add_locations($.from_html(`<button>x</button>`), App[$.FILENAME], [[6, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let cond = $.prop($$props, "cond", 3, true), show = $.prop($$props, "show", 3, true);
	var $$exports = { ...$.legacy_api() };
	var fragment_1 = $.comment();
	$.head("q2w0q4", ($$anchor) => {
		var fragment = $.comment();
		var node = $.first_child(fragment);
		{
			var consequent = ($$anchor) => {
				var meta = root();
				$.append($$anchor, meta);
			};
			$.add_svelte_meta(() => $.if(node, ($$render) => {
				if (show()) $$render(consequent);
			}), "if", App, 9, 4);
		}
		$.append($$anchor, fragment);
	});
	var node_1 = $.first_child(fragment_1);
	{
		var consequent_1 = ($$anchor) => {
			var button = root_1();
			$.append($$anchor, button);
		};
		$.add_svelte_meta(() => $.if(node_1, ($$render) => {
			if (cond()) $$render(consequent_1);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment_1);
	return $.pop($$exports);
}
