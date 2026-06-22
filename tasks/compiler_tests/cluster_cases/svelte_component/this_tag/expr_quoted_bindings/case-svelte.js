import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
import Bar from "./Bar.svelte";
export default function App($$anchor, $$props) {
	let x = $.prop($$props, "x", 8);
	let y = $.prop($$props, "y", 12);
	let z = $.prop($$props, "z", 12);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => x() ? Foo : Bar, ($$anchor, $$component) => {
		$$component($$anchor, {
			get y() {
				return y();
			},
			set y($$value) {
				y($$value);
			},
			get z() {
				return z();
			},
			set z($$value) {
				z($$value);
			},
			$$legacy: true
		});
	});
	$.append($$anchor, fragment);
}
