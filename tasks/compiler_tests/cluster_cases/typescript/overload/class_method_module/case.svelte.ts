export class Store {
	log(a: string): void;
	log(a: number): void;
	log(a: unknown): void {
		console.log(a);
	}
}
