import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	async function g() {
		return 2;
	}
	var $$exports = { ...$.legacy_api() };
	{
		const foo = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var fragment_1 = $.comment();
			var node = $.first_child(fragment_1);
			{
				$.validate_dynamic_element_tag(() => "div");
				$.element(node, () => "div", false, ($$element, $$anchor) => {
					$.attribute_effect($$element, ($0) => ({ title: $0 }), void 0, [async () => (await $.track_reactivity_loss(g()))()]);
				}, void 0, [7, 2]);
			}
			$.append($$anchor, fragment_1);
		});
		$.add_svelte_meta(() => Child($$anchor, {
			foo,
			$$slots: { foo: true }
		}), "component", App, 5, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}
