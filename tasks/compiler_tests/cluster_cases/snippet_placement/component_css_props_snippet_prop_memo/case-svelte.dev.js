App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[8, 2]]);
var root_1 = $.add_locations($.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		const element = $.wrap_snippet(App, function($$anchor, $$arg0) {
			$.validate_snippet_args(...arguments);
			let idx = () => ($$arg0?.()).idx;
			idx();
			var div = root();
			var text = $.child(div, true);
			$.reset(div);
			$.template_effect(() => $.set_text(text, idx()));
			$.append($$anchor, div);
		});
		let $0 = $.derived(() => !$$props.items.length);
		$.css_props(node, () => ({ "--my-var": "baseline" }));
		Child(node.lastChild, {
			get disabled() {
				return $.get($0);
			},
			element,
			$$slots: { element: true }
		});
		$.reset(node);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
