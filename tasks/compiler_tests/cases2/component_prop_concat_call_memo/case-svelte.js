import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
import { getProductName } from "./helpers";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	$.init();
	{
		let $0 = $.derived_safe_equal(getProductName);
		Comp($$anchor, { get title() {
			return `${$.get($0) ?? ""} suffix`;
		} });
	}
	$.pop();
}
