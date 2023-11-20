<script lang="ts" context="module">
    let loadingText: HTMLParagraphElement;

    declare const window: any;
    window.setSpinnerText = function(text: string) {
        loadingText.innerText = text;
    }
</script>

<div class="loader-container">
    <span class="loader"></span>
    <p class="loading-text" bind:this={loadingText}> Loading </p>
</div>


<style>
    .loader-container {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .loading-text {
        font-size: 16px;
        padding-top: 10px;
        color: #fff;
    }

	.loader {
		width: 40px;
		height: 40px;
		border: 5px solid #fff;
		border-bottom-color: transparent;
		border-radius: 50%;
		display: inline-block;
		box-sizing: border-box;
		animation: rotation 1s linear infinite;
	}

	@keyframes rotation {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}
</style>
