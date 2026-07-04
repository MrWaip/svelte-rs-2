import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let tag = $.prop($$props, "tag", 8, "h1");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Foo($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			var fragment_1 = $.comment();
			var node = $.first_child(fragment_1);
			{
				$.validate_dynamic_element_tag(tag);
				$.validate_void_dynamic_element(tag);
				$.element(node, tag, false, ($$element, $$anchor) => {
					var text = $.text("This is default slot");
					$.append($$anchor, text);
				}, void 0, [7, 1]);
			}
			$.append($$anchor, fragment_1);
		}),
		$$slots: {
			default: true,
			other: ($$anchor, $$slotProps) => {
				var fragment_2 = $.comment();
				var node_1 = $.first_child(fragment_2);
				{
					$.validate_dynamic_element_tag(tag);
					$.validate_void_dynamic_element(tag);
					$.element(node_1, tag, false, ($$element_1, $$anchor) => {
						$.attribute_effect($$element_1, () => ({ slot: "other" }));
						var text_1 = $.text("This is other slot");
						$.append($$anchor, text_1);
					}, void 0, [8, 1]);
				}
				$.append($$anchor, fragment_2);
			}
		}
	}), "component", App, 6, 0, { componentTag: "Foo" });
	return $.pop($$exports);
}
