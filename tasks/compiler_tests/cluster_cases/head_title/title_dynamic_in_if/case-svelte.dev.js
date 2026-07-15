import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let condition = $.prop($$props, "condition", 8);
	let name = $.prop($$props, "name", 8);
	var $$exports = { ...$.legacy_api() };
	$.head("q2w0q4", ($$anchor) => {
		var fragment = $.comment();
		var node = $.first_child(fragment);
		{
			var consequent = ($$anchor) => {
				$.deferred_template_effect(() => {
					$.document.title = `Hi ${name() ?? ""}`;
				});
			};
			$.add_svelte_meta(() => $.if(node, ($$render) => {
				if (condition()) $$render(consequent);
			}), "if", App, 7, 1);
		}
		$.append($$anchor, fragment);
	});
	return $.pop($$exports);
}
