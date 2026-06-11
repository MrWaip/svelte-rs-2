import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
const icon = ($$anchor) => {
	var span = root_1();
	$.append($$anchor, span);
};
var root_1 = $.from_html(`<span>hi</span>`);
export default function App($$anchor) {
	Child($$anchor, { get icon() {
		return icon;
	} });
}
