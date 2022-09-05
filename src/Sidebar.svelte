<script lang="ts">
    import { Accordion, AccordionItem } from "sveltestrap";
    import Create from "./lib/Create.svelte";
    import Read from "./lib/Read.svelte";
    import Update from "./lib/Update.svelte";
    import Delete from "./lib/Delete.svelte";
</script>

<Accordion class="accordion" flush>
    <AccordionItem header="Create"><Create /></AccordionItem>
    <AccordionItem header="Read"><Read /></AccordionItem>
    <AccordionItem header="Update"><Update /></AccordionItem>
    <AccordionItem header="Delete"><Delete /></AccordionItem>
</Accordion>
