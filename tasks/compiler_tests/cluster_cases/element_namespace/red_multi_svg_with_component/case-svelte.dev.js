App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
var root = $.add_locations($.from_svg(`<svg></svg><svg></svg><!>`, 1), App[$.FILENAME], [[2, 0], [2, 11]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.sibling($.first_child(fragment), 2);
	$.add_svelte_meta(() => Foo(node, {}), "component", App, 2, 22, { componentTag: "Foo" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
