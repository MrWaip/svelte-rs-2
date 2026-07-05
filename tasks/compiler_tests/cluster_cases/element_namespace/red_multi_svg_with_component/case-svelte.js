import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
var root = $.from_svg(`<svg></svg><svg></svg><!>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var node = $.sibling($.first_child(fragment), 2);
	Foo(node, {});
	$.append($$anchor, fragment);
}
