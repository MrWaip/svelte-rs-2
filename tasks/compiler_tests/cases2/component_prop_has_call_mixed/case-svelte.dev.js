App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function getData() {
		return [
			1,
			2,
			3
		];
	}
	var $$exports = { ...$.legacy_api() };
	{
		let $0 = $.derived(getData);
		$.add_svelte_meta(() => Comp($$anchor, {
			label: "hello",
			get data() {
				return $.get($0);
			},
			get count() {
				return $$props.count;
			}
		}), "component", App, 7, 0, { componentTag: "Comp" });
	}
	return $.pop($$exports);
}
