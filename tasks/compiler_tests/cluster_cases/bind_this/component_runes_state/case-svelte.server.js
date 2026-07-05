import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
export default function App($$renderer) {
	let comp = void 0;
	Comp($$renderer, {});
}
