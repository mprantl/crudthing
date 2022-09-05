<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { Toast, ToastHeader, ToastBody } from "sveltestrap";

    let workerId = "";
    let resultMsg = "";
    let isOpen = false;

    function toggle() {
        isOpen = !isOpen;
    }

    async function delete_worker() {
        resultMsg = await invoke("delete_worker", { workerId });
        if (resultMsg.includes("Error")) {
            alert("Unable to find: " + workerId + "\n" + resultMsg);
        } else {
            toggle();
        }
    }
</script>

<Toast class="position-fixed bottom-0 end-0 p-3" {isOpen}>
    <ToastHeader {toggle}>Deletion successful!</ToastHeader>
    <ToastBody
        >Deleted id: "{workerId}" <br />
        {resultMsg}</ToastBody
    >
</Toast>

<input
    id="worker-id-input"
    placeholder="Delete entry by ID..."
    bind:value={workerId}
/>
<button on:click={delete_worker}> Delete </button>
