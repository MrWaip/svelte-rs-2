import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
import { BRAND } from "./brand";
import { getName } from "./helpers";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	{
		let $0 = $.derived(() => BRAND);
		let $1 = $.derived(getName);
		Comp($$anchor, { get title() {
			return `prefix ${$.get($0) ?? ""}${$.get($1) ?? ""}`;
		} });
	}
	$.pop();
}
