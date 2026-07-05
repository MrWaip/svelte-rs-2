App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
import B from "./B.svelte";
var root = $.add_locations($.from_html(`<div>c</div>`), App[$.FILENAME], [[16, 2]]);
var root_1 = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let data = null;
	let x = null;
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		const inner = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			$.next();
			var text = $.text();
			text.nodeValue = "";
			$.append($$anchor, text);
		});
		$.add_svelte_meta(() => A(node, {
			inner,
			$$slots: { inner: true }
		}), "component", App, 8, 0, { componentTag: "A" });
	}
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => B(node_1, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			var fragment_2 = $.comment();
			var node_2 = $.first_child(fragment_2);
			{
				var consequent = ($$anchor) => {
					var div = root();
					$.append($$anchor, div);
				};
				$.add_svelte_meta(() => $.if(node_2, ($$render) => {
					if (data) $$render(consequent);
				}), "if", App, 15, 1);
			}
			$.append($$anchor, fragment_2);
		}),
		$$slots: { default: true }
	}), "component", App, 14, 0, { componentTag: "B" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
