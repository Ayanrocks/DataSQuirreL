<script>
  import { notificationMsg } from '../stores';
  import {
    NOTIFICATION_TYPE_ERROR,
    NOTIFICATION_TYPE_SUCCESS,
    NOTIFICATION_TYPE_WARNING,
  } from '../constants/constants.js';

  let msg = '';
  let type = '';

  notificationMsg.subscribe((val) => {
    if (msg == '') {
      setTimeout(() => {
        console.log('Registering after 5sec');
        notificationMsg.set({ type: '', message: '' });
      }, 5000);
    }
    msg = val.message;
    type = val.type;
  });

  function hideMsg() {
    notificationMsg.set({ type: '', message: '' });
  }

  function returnNotificationElemnt() {
    if (msg != '') {
      return '';
    }
    switch (type) {
      case NOTIFCATION_TYPE_SUCCESS:
        return ` <div class="notification is-primary" id="primary-notification">
                    <button class="delete" on:click={hideMsg} />
                    {msg}
                  </div>`;
      case NOTIFICATION_TYPE_ERROR:
        return ` <div class="notification is-warning" id="primary-notification">
                    <button class="delete" on:click={hideMsg} />
                    {msg}
                  </div>`;
      case NOTIFICATION_TYPE_ERROR:
        return ` <div class="notification is-danger" id="primary-notification">
                    <button class="delete" on:click={hideMsg} />
                    {msg}
                  </div>`;

      default:
        return '';
    }
  }
</script>

{msg == '' ? '' : returnNotificationElemnt()}

<style>
  .notification {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    transition: 0.5s;
    text-align: center;
    z-index: 999;
  }
</style>
