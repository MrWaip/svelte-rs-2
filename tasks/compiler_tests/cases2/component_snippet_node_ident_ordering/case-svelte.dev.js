App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<!> <div class="cap"><!></div>`, 1), App[$.FILENAME], [[9, 12]]);
var root_1 = $.add_locations($.from_html(`<div><!></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	var node = $.child(div);
	{
		const footer = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var fragment = root();
			var node_1 = $.first_child(fragment);
			$.add_svelte_meta(() => $.component(node_1, () => $$props.Btn, ($$anchor, Btn_1) => {
				Btn_1($$anchor, {});
			}), "component", App, 8, 12, { componentTag: "Btn" });
			var div_1 = $.sibling(node_1, 2);
			var node_2 = $.child(div_1);
			$.add_svelte_meta(() => $.component(node_2, () => $$props.Cap, ($$anchor, Cap_1) => {
				Cap_1($$anchor, {});
			}), "component", App, 9, 29, { componentTag: "Cap" });
			$.reset(div_1);
			$.append($$anchor, fragment);
		});
		$.add_svelte_meta(() => $.component(node, () => $$props.Layout, ($$anchor, Layout_1) => {
			Layout_1($$anchor, {
				footer,
				$$slots: { footer: true }
			});
		}), "component", App, 6, 4, { componentTag: "Layout" });
	}
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
