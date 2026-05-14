import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
import { getProductName } from "./helpers";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	{
		let $0 = $.derived(getProductName);
		Comp($$anchor, { get title() {
			return `x0${$.get($0) ?? ""}`;
		} });
	}
	$.pop();
}
