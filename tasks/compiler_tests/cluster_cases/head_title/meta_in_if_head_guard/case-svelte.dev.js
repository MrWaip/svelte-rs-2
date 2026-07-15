import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<meta id="m" name="x" content="y"/>`), App[$.FILENAME], [[7, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let condition = $.prop($$props, "condition", 8);
	var $$exports = { ...$.legacy_api() };
	$.head("q2w0q4", ($$anchor) => {
		var fragment = $.comment();
		var node = $.first_child(fragment);
		{
			var consequent = ($$anchor) => {
				var meta = root();
				$.append($$anchor, meta);
			};
			$.add_svelte_meta(() => $.if(node, ($$render) => {
				if (condition()) $$render(consequent);
			}), "if", App, 6, 1);
		}
		$.append($$anchor, fragment);
	});
	return $.pop($$exports);
}
