App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function fn1() {
		return "a";
	}
	function fn2() {
		return "b";
	}
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived(fn1);
		let $1 = $.derived(fn2);
		$.add_svelte_meta(() => Comp($$anchor, {
			get a() {
				return $.get($0);
			},
			get b() {
				return $.get($1);
			}
		}), "component", App, 7, 0, { componentTag: "Comp" });
	}
	return $.pop($$exports);
}
