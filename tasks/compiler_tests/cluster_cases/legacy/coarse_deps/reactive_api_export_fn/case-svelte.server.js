import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $$props["a"];
	function foo() {}
	$: {
		a;
		foo();
	}
	$.bind_props($$props, {
		a,
		foo
	});
}
