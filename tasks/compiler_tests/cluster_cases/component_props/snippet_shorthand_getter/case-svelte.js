import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
const icon = ($$anchor) => {
	var span = root();
	$.append($$anchor, span);
};
var root = $.from_html(`<span>hi</span>`);
export default function App($$anchor) {
	Child($$anchor, { get icon() {
		return icon;
	} });
}
