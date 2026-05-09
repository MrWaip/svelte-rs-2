import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
import { foo } from "./x.js";
export default function App($$anchor, $$props) {
	Comp($$anchor, { get p() {
		return foo;
	} });
}
