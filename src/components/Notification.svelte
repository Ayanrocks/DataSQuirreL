<script lang="ts">
  import { notificationMsg } from '../stores';
  import { onMount } from 'svelte';
  import { slide } from 'svelte/transition';
  import {
    NOTIFICATION_TYPE_ERROR,
    NOTIFICATION_TYPE_SUCCESS,
    NOTIFICATION_TYPE_WARNING,
  } from '../constants/constants.js';

  let msg = '';
  let type = '';
  let alreadyRegistered = false;
  let notificationClearTimeout: number;

  notificationMsg.subscribe((val) => {
    if (msg == '' && !alreadyRegistered && val.message != '') {
      console.log('Registering Clear Event');
      let timeout = 3 * 1000; // 3 sec for success messages
      if (val.type != NOTIFICATION_TYPE_SUCCESS) {
        timeout = 6 * 1000;
      }
      notificationClearTimeout = setTimeout(() => {
        console.log('Cleared after ', timeout, 's');
        notificationMsg.set({ type: '', message: '' });
        alreadyRegistered = false;
      }, timeout);
      alreadyRegistered = true;
    }
    msg = val.message;
    type = val.type;
  });

  function getShadowClass() {
    if (type == NOTIFICATION_TYPE_SUCCESS) {
      return 'notification-container success-shadow';
    } else if (type == NOTIFICATION_TYPE_WARNING) {
      return 'notification-container warning-shadow';
    } else if (type == NOTIFICATION_TYPE_ERROR) {
      return 'notification-container error-shadow';
    } else {
      return 'notification-container';
    }
  }

  function hideMsg() {
    notificationMsg.set({ type: '', message: '' });
    alreadyRegistered = false;
    clearTimeout(notificationClearTimeout);
  }
</script>

{#if msg != ''}
  <div class={getShadowClass()} transition:slide>
    {#if type == NOTIFICATION_TYPE_SUCCESS}
      <div class="notification is-primary" id="primary-notification">
        <button class="delete" on:click={hideMsg} />
        <div class="notification-body">
          {msg}
        </div>
      </div>
    {:else if type == NOTIFICATION_TYPE_WARNING}
      <div class="notification is-warning" id="primary-notification">
        <button class="delete" on:click={hideMsg} />
        <div class="notification-body">
          {msg}
        </div>
      </div>
    {:else if type == NOTIFICATION_TYPE_ERROR}
      <div class="notification is-danger" id="primary-notification">
        <button class="delete" on:click={hideMsg} />
        <div class="notification-body">
          {msg}
        </div>
      </div>
    {/if}
  </div>
{/if}

<style>
  .notification-container {
    position: fixed;
    top: 3rem;
    right: 1rem;
    z-index: 999;
    width: 20rem;
    transition: 0.5s;
  }

  .warninng-shadow {
    border-radius: 4px;
    -webkit-box-shadow: 1px 5px 7px 0px rgba(255, 227, 150, 0.5);
    -moz-box-shadow: 1px 5px 7px 0px rgba(255, 227, 150, 0.5);
    box-shadow: 1px 5px 7px 0px rgba(255, 227, 150, 0.5);
  }

  .success-shadow {
    border-radius: 4px;
    -webkit-box-shadow: 1px 6px 7px 0px rgba(26, 214, 186, 0.5);
    -moz-box-shadow: 1px 6px 7px 0px rgba(26, 214, 186, 0.5);
    box-shadow: 1px 6px 7px 0px rgba(26, 214, 186, 0.5);
  }

  .error-shadow {
    border-radius: 4px;
    -webkit-box-shadow: 1px 5px 7px 0px rgba(242, 89, 119, 0.5);
    -moz-box-shadow: 1px 5px 7px 0px rgba(242, 89, 119, 0.5);
    box-shadow: 1px 5px 7px 0px rgba(242, 89, 119, 0.5);
  }

  .notification-body {
    padding: 1rem;
    word-wrap: break-word;
  }
</style>
