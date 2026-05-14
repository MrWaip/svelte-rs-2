import * as $ from "svelte/internal/client";
import { getProductName } from "./helpers";
var root = $.from_html(`<div>hi</div>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	var div = root();
	$.template_effect(($0) => $.set_attribute(div, "title", `x0${$0 ?? ""}`), [() => getProductName()]);
	$.append($$anchor, div);
	$.pop();
}
