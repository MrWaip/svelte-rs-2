import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
import { foo } from "./x.js";
export default function App($$renderer, $$props) {
	let { x } = $$props;
	Comp($$renderer, { p: foo });
}
