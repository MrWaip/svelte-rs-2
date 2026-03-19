import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let host = $$props.$$host;
}
customElements.define("my-element", $.create_custom_element(App, {}, [], [], { mode: "open" }));
