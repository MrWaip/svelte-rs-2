import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
var root = $.add_locations($.from_html(`<span>default</span>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Foo($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			var span = root();
			$.append($$anchor, span);
		}),
		$$slots: {
			default: true,
			other: ($$anchor, $$slotProps) => {
				var fragment_1 = $.comment();
				var node = $.first_child(fragment_1);
				{
					$.validate_dynamic_element_tag(() => "span");
					$.validate_void_dynamic_element(() => "span");
					$.element(node, () => "span", false, ($$element, $$anchor) => {
						$.attribute_effect($$element, () => ({ slot: "other" }));
						var text = $.text("This is other slot");
						$.append($$anchor, text);
					}, void 0, [7, 1]);
				}
				$.append($$anchor, fragment_1);
			}
		}
	}), "component", App, 5, 0, { componentTag: "Foo" });
	return $.pop($$exports);
}
