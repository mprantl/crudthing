<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { Column, Table } from "sveltestrap";

    let table = [];

    async function fetch_table() {
        table = await invoke("fetch_table");
    }
    onMount(async () => {
        table = await invoke("fetch_table");
    });
</script>

<Table
    class="table table-borderless table-striped table-hover"
    rows={table}
    let:row
>
    <Column header="ID" width="2rem">
        {row[0]}
    </Column>
    <Column header="First Name" width="2rem">
        {row[1]}
    </Column>
</Table>

<button on:click={fetch_table}>Update</button>
