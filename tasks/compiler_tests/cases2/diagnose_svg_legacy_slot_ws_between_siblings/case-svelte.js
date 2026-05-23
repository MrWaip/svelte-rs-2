import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root_2 = $.from_svg(`<g><path d="M1"></path></g><g><path d="M2"></path></g>`, 1);
export default function App($$anchor) {
	Child($$anchor, { $$slots: { header: ($$anchor, $$slotProps) => {
		var fragment_1 = root_2();
		$.next();
		$.append($$anchor, fragment_1);
	} } });
}
