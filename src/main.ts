import { invoke } from "@tauri-apps/api/tauri";

function main() {
  let platform: 'linux' | 'windows';
  function onSubmit(ev: Event) {
    ev.preventDefault();
    let url: string | undefined = getUrl();
    if (url == null || url.trim().length === 0) {
      alert("URL is empty");
    } else {
      invoke("edit_vhs_file", {
        address: url,
      }).then(() => alert("Server set succesfully!")).catch(alert);
    }
  }

  function onSubmitMod(ev: Event) {
    ev.preventDefault();
    let url: string | undefined = getUrl();
    if (url == null || url.trim().length === 0) {
      alert("URL is empty");
    } else {
      invoke("edit_vhs_and_add_mod", {
        address: url,
      }).then(alert).catch(alert);
    }
  }

  function getUrl() {
    const selected: string = document.forms.namedItem('form')!.server.value;
    let url: string | undefined;
    switch (selected) {
      case 'self':
        url = platform === 'windows' ? "127.0.0.1" : "http://localhost:12478/vhs-%s/%s/%s/?guid=%s";
        break;
      case 'other':
        url = (document.getElementById('other-text') as HTMLInputElement | null)?.value;
        break;
      case 'main':
      default:
        url = platform === 'windows' ? "173.249.51.206" : "https://apps.luismayo.com/vhs-%s/%s/%s/?guid=%s";
    }
    return url;
  }

  function onFormChange() {
    const serverIsOther = (document.getElementById('other') as HTMLInputElement | null)?.checked;
    const textField = document.getElementById("other-text") as HTMLInputElement | null;
    if (textField) {
      textField.disabled = !serverIsOther;
    }
  }

  function onRestore() {
    invoke("restore_backup_handler").then(() => { alert("Backup restored succesfully. Remember you can verify files on Steam"); init(); })
      .catch(alert);
  }

  window.addEventListener("DOMContentLoaded", () => {
    const form = document.getElementById('form');
    if (form) {
      form.addEventListener('change', onFormChange);
      form.addEventListener('submit', onSubmit);
    }
    const restoreButton = document.getElementById('restore');
    if (restoreButton) {
      restoreButton.addEventListener('click', onRestore);
    }
    const modWithBots = document.getElementById('mod_server_with_mods');
    if (modWithBots) {
      modWithBots.addEventListener('click', onSubmitMod);
    }
  });

  function init() {
    setTimeout(() => {
      invoke<{ platform: 'windows' | 'linux', msgs: { success: boolean, msg: string }[] }>('init').then((msgs) => {
        const container = document.getElementById('msg-container')!;
        container.innerHTML = '';
        platform = msgs.platform;
        for (const msg of msgs.msgs) {
          const span = document.createElement('div');
          span.textContent = (msg.success ? '✅ ' : '❌ ') + msg.msg;
          container.appendChild(span);
        }
      });
    }, 0)
  }

  init();
}

main();
