App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span>a</span>`), App[$.FILENAME], [[8, 12]]);
var root_1 = $.add_locations($.from_html(`<!> <p>tail</p>`, 1), App[$.FILENAME], [[10, 8]]);
var root_2 = $.add_locations($.from_html(`<div><!></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root_2();
	var node = $.child(div);
	$.add_svelte_meta(() => Cmp(node, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			var fragment = root_1();
			var node_1 = $.first_child(fragment);
			{
				var consequent = ($$anchor) => {
					var span = root();
					$.append($$anchor, span);
				};
				$.add_svelte_meta(() => $.if(node_1, ($$render) => {
					if ($$props.x) $$render(consequent);
				}), "if", App, 7, 8);
			}
			$.next(2);
			$.append($$anchor, fragment);
		}),
		$$slots: { default: true }
	}), "component", App, 6, 4, { componentTag: "Cmp" });
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
