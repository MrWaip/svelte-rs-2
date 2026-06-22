import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
var root_1 = $.from_html(`<span>default</span>`);
export default function App($$anchor) {
	Foo($$anchor, {
		children: ($$anchor, $$slotProps) => {
			var span = root_1();
			$.append($$anchor, span);
		},
		$$slots: {
			default: true,
			other: ($$anchor, $$slotProps) => {
				var fragment_1 = $.comment();
				var node = $.first_child(fragment_1);
				$.element(node, () => "span", false, ($$element, $$anchor) => {
					$.attribute_effect($$element, () => ({ slot: "other" }));
					var text = $.text("This is other slot");
					$.append($$anchor, text);
				});
				$.append($$anchor, fragment_1);
			}
		}
	});
}
