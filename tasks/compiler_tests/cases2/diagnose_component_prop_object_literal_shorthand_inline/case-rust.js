import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	const handler = () => {};
	Child($$anchor, { props: { handler } });
}
