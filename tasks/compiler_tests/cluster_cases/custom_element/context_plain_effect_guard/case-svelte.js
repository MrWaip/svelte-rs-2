import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let props = $.rest_props($$props, rest_excludes);
	$.user_effect(() => console.log(props));
	$.pop();
}
