<script>
class Outer {
    constructor() {
        const Inner = class {
            constructor() {
                this.a = $state(0);
            }
        };
        this.b = $state(1);
    }
}
</script>
