<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { Toast, ToastBody, ToastHeader } from "sveltestrap";

    let workerId = "";
    let resultMsg = "";
    let isOpen = false;

    function toggle() {
        isOpen = !isOpen;
    }

    async function checkid() {
        resultMsg = await invoke("check_worker_id", { workerId });
        if (resultMsg.includes("Error")) {
            alert(resultMsg);
        } else {
            toggle();
        }
    }
</script>

<Toast class="position-fixed bottom-0 end-0 p-3" {isOpen}>
    <ToastHeader {toggle}>Check successful!</ToastHeader>
    <ToastBody>"{workerId}" is valid! <br /> name: "{resultMsg}"</ToastBody>
</Toast>

<input id="greet-input" placeholder="Enter ID..." bind:value={workerId} />
<button on:click={checkid}> Check </button>
