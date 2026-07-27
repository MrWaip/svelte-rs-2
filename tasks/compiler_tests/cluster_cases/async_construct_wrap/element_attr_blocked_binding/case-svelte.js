import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<div></div> <!>`, 1);
export default function App($$anchor) {
	var x;
	var $$promises = $.run([() => Promise.resolve(), () => x = 2]);
	var fragment = root();
	var div = $.first_child(fragment);
	var node = $.sibling(div, 2);
	$.async(node, [$$promises[1]], void 0, ($$anchor) => {
		Child(node, {
			get a() {
				return () => x;
			},
			get b() {
				return { k: x };
			}
		});
	});
	$.template_effect(() => {
		$.set_attribute(div, "title", x);
		$.set_class(div, 1, "a2b");
	}, void 0, void 0, [$$promises[1]]);
	$.append($$anchor, fragment);
}
