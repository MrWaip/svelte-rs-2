import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
export default function App($$renderer) {
	function fn1() {
		return "a";
	}
	function fn2() {
		return "b";
	}
	Comp($$renderer, {
		a: fn1(),
		b: fn2()
	});
}
