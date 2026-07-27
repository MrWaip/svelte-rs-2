import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<div></div> <!>`, 1), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var x;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => x = 2]);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	var node = $.sibling(div, 2);
	$.async(node, [$$promises[1]], void 0, ($$anchor) => {
		$.add_svelte_meta(() => Child(node, {
			get a() {
				return () => x;
			},
			get b() {
				return { k: x };
			}
		}), "component", App, 9, 0, { componentTag: "Child" });
	});
	$.template_effect(() => {
		$.set_attribute(div, "title", x);
		$.set_class(div, 1, "a2b");
	}, void 0, void 0, [$$promises[1]]);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
