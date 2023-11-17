import { invoke } from "@tauri-apps/api/tauri";

function onSubmit(ev: Event) {
  ev.preventDefault();
  const selected: string = document.forms.namedItem('form')!.server.value;
  let url: string | undefined;
  switch(selected) {
    case 'self':
      url = "http://localhost:12478/vhs-%s/%s/%s/?guid=%s";
      break;
    case 'other':
      url = (document.getElementById('other-text') as HTMLInputElement | null)?.value
      break;
    case 'main':
    default:
      url = "https://apps.luismayo.com/vhs-%s/%s/%s/?guid=%s"
  }
  if (url == null || url.trim().length === 0) {
    alert("URL is empty");
  } else {
    invoke("edit_vhs_file", {
      address: url,
    }).then(() => alert("Server set succesfully!")).catch(alert);
  }
}

function onFormChange() {
  const serverIsOther = (document.getElementById('other') as HTMLInputElement | null)?.checked;
  const textField = document.getElementById("other-text") as HTMLInputElement | null;
  if (textField) {
    textField.disabled = !serverIsOther;
  }
}

function onRestore() {
  invoke("restore_backup_handler").then(() => alert("Backup restored succesfully. Remember you can verify files on Steam"))
  .catch(alert);
}

window.addEventListener("DOMContentLoaded", () => {
  const form  = document.getElementById('form');
  if (form) {
    form.addEventListener('change', onFormChange);
    form.addEventListener('submit', onSubmit);
  }
  const restoreButton = document.getElementById('restore');
  if (restoreButton) {
    restoreButton.addEventListener('click', onRestore);
  }
});
