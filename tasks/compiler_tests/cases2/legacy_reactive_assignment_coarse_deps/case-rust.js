import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p class="x"></p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let items = $.prop($$props, "items", 24, () => [{ value: 1 }]);
	let extra = 2;
	$: prop_total = items()[0].value + extra;
	$: props_items = $$props.items[0].value;
	$: rest_class = $$restProps.class ?? "none";
	var $$exports = { extra };
	var p = root();
	p.textContent = `${prop_total ?? ""}-${props_items ?? ""}-${rest_class ?? ""}`;
	$.append($$anchor, p);
	return $.pop($$exports);
}
