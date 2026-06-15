import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
import Bar from "./Bar.svelte";
export default function App($$anchor, $$props) {
	let x = $.prop($$props, "x", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => x() ? Foo : Bar, ($$anchor, $$component) => {
		$$component($$anchor, { answer: 42 });
	});
	$.append($$anchor, fragment);
}
