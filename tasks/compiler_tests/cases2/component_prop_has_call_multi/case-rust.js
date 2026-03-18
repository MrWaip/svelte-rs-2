import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor) {
	function fn1() {
		return "a";
	}
	function fn2() {
		return "b";
	}
	{
		let $0 = $.derived(fn1);
		let $1 = $.derived(fn2);
		Comp($$anchor, {
			get a() {
				return $.get($0);
			},
			get b() {
				return $.get($1);
			}
		});
	}
}
