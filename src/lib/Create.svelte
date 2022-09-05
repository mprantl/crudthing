<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { Toast, ToastBody, ToastHeader } from "sveltestrap";

    let workerId = "";
    let workerName = "";
    let resultMsg = "";
    let isOpen = false;

    function toggle() {
        isOpen = !isOpen;
    }

    async function insert() {
        resultMsg = await invoke("create", { workerId, workerName });
        if (resultMsg.includes("Error")) {
            alert(resultMsg);
        } else {
            toggle();
        }
    }
</script>

<Toast class="position-fixed bottom-0 end-0 p-3" {isOpen}>
    <ToastHeader {toggle}>Insert successful!</ToastHeader>
    <ToastBody
        >Inserted id: "{workerId}" name: "{workerName}" <br />
        {resultMsg}</ToastBody
    >
</Toast>

<input id="worker-id-input" placeholder="Enter a ID..." bind:value={workerId} />

<input
    id="worker-name-input"
    placeholder="Enter a name"
    bind:value={workerName}
/>
<button on:click={insert}> Insert </button>
