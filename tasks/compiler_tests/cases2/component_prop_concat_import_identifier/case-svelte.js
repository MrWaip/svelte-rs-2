import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
import { BRAND } from "./brand";
export default function App($$anchor) {
	Comp($$anchor, { get title() {
		return `prefix ${BRAND ?? ""}`;
	} });
}
