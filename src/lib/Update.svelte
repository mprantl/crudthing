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
        resultMsg = await invoke("update", { workerId, workerName });
        if (resultMsg.includes("Error")) {
            alert("Unable to find: " + workerId + "\n" + resultMsg);
        } else {
            toggle();
        }
    }
</script>

<Toast class="position-fixed bottom-0 end-0 p-3" {isOpen}>
    <ToastHeader {toggle}>Update successful!</ToastHeader>
    <ToastBody
        >Updated id: "{workerId}" new name: "{workerName}" <br />
        {resultMsg}</ToastBody
    >
</Toast>

<input id="worker-id-input" placeholder="Enter a ID..." bind:value={workerId} />
<input
    id="worker-name-input"
    placeholder="Update a name"
    bind:value={workerName}
/>
<button on:click={insert}> Update </button>
