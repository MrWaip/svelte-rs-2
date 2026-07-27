App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const co = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	var b = root();
	$.append($$anchor, b);
});
var root = $.add_locations($.from_html(`<b>C</b>`), App[$.FILENAME], [[5, 15]]);
var option_content = $.add_locations($.from_html(`<span>M</span>`, 1), App[$.FILENAME], [[7, 16]]);
var root_1 = $.add_locations($.from_html(`<!> <select><option><!></option></select>`, 1), App[$.FILENAME], [[
	7,
	0,
	[[7, 8]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let show = true;
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			$.add_svelte_meta(() => co($$anchor), "render", App, 6, 10);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (show) $$render(consequent);
		}), "if", App, 6, 0);
	}
	var select = $.sibling(node, 2);
	var option = $.child(select);
	$.customizable_select(option, () => {
		var anchor = $.child(option);
		var fragment_2 = option_content();
		$.append(anchor, fragment_2);
	});
	$.reset(select);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
