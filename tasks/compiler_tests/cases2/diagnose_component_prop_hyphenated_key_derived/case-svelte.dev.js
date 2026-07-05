App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<button> </button> <!>`, 1), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.child(button, true);
	$.reset(button);
	var node = $.sibling(button, 2);
	{
		let $0 = $.derived(() => !$.get(count));
		$.add_svelte_meta(() => Child(node, {
			get "aria-disabled"() {
				return $.get($0);
			},
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				$.next();
				var text_1 = $.text("hi");
				$.append($$anchor, text_1);
			}),
			$$slots: { default: true }
		}), "component", App, 8, 0, { componentTag: "Child" });
	}
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, function click() {
		return $.update(count);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
