import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	Table($$anchor, { get items() {
		return $$props.data;
	} });
}
