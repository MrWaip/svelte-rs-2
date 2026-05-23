import * as $ from "svelte/internal/client";
import { act } from "./act";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let ref;
	let attrs = {};
	var div = root();
	$.attribute_effect(div, () => ({ ...attrs }));
	$.action(div, ($$node) => act?.($$node));
	$.bind_this(div, ($$value) => ref = $$value, () => ref);
	$.append($$anchor, div);
}
