App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[9, 3]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	{
		const body = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var fragment_1 = $.comment();
			var node = $.first_child(fragment_1);
			{
				var consequent = ($$anchor) => {
					var div = root();
					{
						const inner = $.wrap_snippet(App, function($$anchor) {
							$.validate_snippet_args(...arguments);
							$.next();
							var text = $.text();
							$.template_effect(() => $.set_text(text, $$props.data?.flag?.text));
							$.append($$anchor, text);
						});
					}
					$.append($$anchor, div);
				};
				$.add_svelte_meta(() => $.if(node, ($$render) => {
					if ($$props.data?.flag) $$render(consequent);
				}), "if", App, 8, 2);
			}
			$.append($$anchor, fragment_1);
		});
		$.add_svelte_meta(() => Outer($$anchor, {
			body,
			$$slots: { body: true }
		}), "component", App, 6, 0, { componentTag: "Outer" });
	}
	return $.pop($$exports);
}
