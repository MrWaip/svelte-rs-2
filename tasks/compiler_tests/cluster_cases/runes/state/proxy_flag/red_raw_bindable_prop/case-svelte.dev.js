App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<!> <p> </p>`, 1), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let object = $.tag($.state({ count: 0 }), "object");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => Child(node, {
		get object() {
			return $.get(object);
		},
		set object($$value) {
			$.set(object, $$value);
		}
	}), "component", App, 6, 0, { componentTag: "Child" });
	var p = $.sibling(node, 2);
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(object).count));
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
