<script lang="ts">
  import MainLogo from "../assets/MainLogo.png";

  let { isOpen = false, onClose }: { isOpen: boolean; onClose: () => void } =
    $props();

  function handleClose() {
    onClose();
  }

  // Use the global variables injected by Vite
  const version = __APP_VERSION__;
  const commitHash = __GIT_HASH__;
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={handleClose}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h2 class="title">About DataSquirrel</h2>
        <button class="close-btn" onclick={handleClose}>✕</button>
      </div>

      <div class="modal-body">
        <div class="logo-container">
          <img src={MainLogo} alt="DataSQuirreL Logo" />
        </div>

        <div class="info-section">
          <h3>DataSQuirreL</h3>
          <p class="description">
            A lightweight database viewer with a modern UI. Supporting multiple
            database connections seamlessly.
          </p>

          <div class="version-info">
            <div class="version-row">
              <span class="label">Version:</span>
              <span class="value">v{version}</span>
            </div>
            <div class="version-row">
              <span class="label">Commit Hash:</span>
              <span class="value monospaced">{commitHash}</span>
            </div>
          </div>

          <div class="footer">
            <p>Made with ❤️ by Ayan Banerje</p>
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}

<style lang="postcss">
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: rgba(0, 0, 0, 0.4);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 9999;
    backdrop-filter: blur(2px);
  }

  .modal-content {
    background-color: #ffffff;
    border-radius: 12px;
    width: 450px;
    max-width: 90vw;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    animation: slideUp 0.3s ease-out forwards;
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 24px;
    border-bottom: 1px solid #eaeaea;
    background-color: #fafafa;
  }

  .title {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: #333;
    font-family: "Outfit", sans-serif;
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 1.2rem;
    cursor: pointer;
    color: #888;
    transition: color 0.2s ease;
  }

  .close-btn:hover {
    color: #333;
  }

  .modal-body {
    padding: 30px 24px;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
  }

  .logo-container {
    width: 250px;
    margin-bottom: 20px;
  }

  :global(.logo-container img) {
    width: 100%;
    height: auto;
  }

  .info-section {
    display: flex;
    flex-direction: column;
    gap: 15px;
    width: 100%;
  }

  .info-section h3 {
    margin: 0;
    font-size: 1.5rem;
    color: #222;
    font-family: "Outfit", sans-serif;
  }

  .description {
    margin: 0;
    color: #555;
    font-size: 0.95rem;
    line-height: 1.5;
  }

  .version-info {
    margin-top: 10px;
    background: #f8f9fa;
    border: 1px solid #e9ecef;
    border-radius: 8px;
    padding: 12px 16px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    text-align: left;
  }

  .version-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.9rem;
  }

  .label {
    color: #666;
    font-weight: 500;
  }

  .value {
    color: #222;
    font-weight: 600;
  }

  .monospaced {
    font-family: "Jetbrains Mono", monospace;
    background: #e9ecef;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 0.85rem;
    letter-spacing: 0.05em;
  }

  .footer {
    margin-top: 25px;
    font-size: 0.85rem;
    color: #aaa;
  }
</style>
