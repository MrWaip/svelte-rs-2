import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
var root = $.add_locations($.from_html(`<span>child</span>`), App[$.FILENAME], [[10, 8]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let current = A;
	let cond = false;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.component(node, () => current, ($$anchor, $$component) => {
		$$component($$anchor, {
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				var fragment_1 = $.comment();
				var node_1 = $.first_child(fragment_1);
				{
					var consequent = ($$anchor) => {
						var span = root();
						$.append($$anchor, span);
					};
					$.add_svelte_meta(() => $.if(node_1, ($$render) => {
						if (cond) $$render(consequent);
					}), "if", App, 9, 4);
				}
				$.append($$anchor, fragment_1);
			}),
			$$slots: { default: true }
		});
	}), "component", App, 8, 0, { componentTag: "svelte:component" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
