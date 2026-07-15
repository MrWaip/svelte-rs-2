import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8);
	let b = $.prop($$props, "b", 8);
	var $$exports = { ...$.legacy_api() };
	$.head("q2w0q4", ($$anchor) => {
		var fragment = $.comment();
		var node = $.first_child(fragment);
		{
			var consequent_1 = ($$anchor) => {
				var fragment_1 = $.comment();
				var node_1 = $.first_child(fragment_1);
				{
					var consequent = ($$anchor) => {
						$.effect(() => {
							$.document.title = "deep";
						});
					};
					$.add_svelte_meta(() => $.if(node_1, ($$render) => {
						if (b()) $$render(consequent);
					}), "if", App, 8, 2);
				}
				$.append($$anchor, fragment_1);
			};
			$.add_svelte_meta(() => $.if(node, ($$render) => {
				if (a()) $$render(consequent_1);
			}), "if", App, 7, 1);
		}
		$.append($$anchor, fragment);
	});
	return $.pop($$exports);
}
