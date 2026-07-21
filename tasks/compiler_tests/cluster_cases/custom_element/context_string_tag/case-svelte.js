import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"$$host"
]);
export default function App($$anchor, $$props) {
	let props = $.rest_props($$props, rest_excludes);
}
customElements.define("my-el", $.create_custom_element(App, {}, [], [], { mode: "open" }));
