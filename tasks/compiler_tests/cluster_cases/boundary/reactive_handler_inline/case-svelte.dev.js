App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>err</p>`), App[$.FILENAME], [[7, 11]]);
var root_1 = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let error = $.tag($.state(void 0), "error");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.boundary(node, { onerror: (e) => $.set(error, e, true) }, ($$anchor) => {
		$.next();
		var text = $.text("x");
		$.append($$anchor, text);
	});
	var node_1 = $.sibling(node, 2);
	{
		var consequent = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		$.add_svelte_meta(() => $.if(node_1, ($$render) => {
			if ($.get(error)) $$render(consequent);
		}), "if", App, 7, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
