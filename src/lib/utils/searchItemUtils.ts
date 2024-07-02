import { invoke, transformCallback } from "@tauri-apps/api/core";
import { trackEvent } from '@aptabase/web';
import moment from 'moment';

export async function openFileFolder(url: string) {
  trackEvent('click:openFile');
  invoke('open_folder_containing_file', { filePath: url });
}

export async function openFile(url: string) {
  trackEvent('click:openFile');
  invoke('open_file_or_folder', { filePath: url });
}

export function formatPath(url: string): string {
  const parts = url.split('/'); // Split the url into components
  const length = parts.length;

  if (length > 3) {
    // Take the two directories just before the file name and prepend '...'
    return '.../' + parts.slice(length - 3, length - 1).join('/') + '/';
  } else {
    // If the url is already short, return it as is
    return url;
  }
}

export function formatUpdatedTime(unixTime: number): string {
  if (unixTime === 0) {
    return 'Never';
  }
  let unixToJs = new Date(unixTime*1000);
  const updatedMoment = moment(unixToJs);
  const today = moment();
  const yesterday = moment().subtract(1, 'days');

  if (updatedMoment.isSame(today, 'day')) {
    // If the update was today, return the time
    return updatedMoment.format('h:mm A');
  } else if (updatedMoment.isSame(yesterday, 'day')) {
    // If the update was yesterday, return 'Yesterday'
    return 'Yesterday';
  } else {
    // Otherwise, return the date
    return updatedMoment.format('YYYY-MM-DD');
  }
}

export function startDragging(filepath: string) {
  // const image64 = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAA7AAAAOwBeShxvQAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAADySURBVFiF7dcxSgNRFIXhT4Wx1NrCFCK4iEiCWxBcgjsQscteLMQlKCksLCLYWbkE3cEEJBaTgL5MzGReAhb3h9cc3tz7w0wxh3k6GKLEpOV5R7dmdiOGuEHR8vkBHvGBfpsBZcbymcBgunypxHZNVmCcITDjCee4x9kqAuvkeSpx95dEyiRz6SVuk6yPT5yml7cWCNTlTdnHK0Z4+5F3cYyTTQvAHi5wlOTXTWbnvoKVZm/6I1xKCIRACIRACIRACIRACPxLgbG8araIXVXt+8VOzcUeDvCCrzUtL3Cl+iVPS8scHVW7zann6SnxgMN02Ter0UNOfhP2XAAAAABJRU5ErkJggg==";
  const fileImage64 = "data:image/png;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9ImN1cnJlbnRDb2xvciIgc3Ryb2tlLXdpZHRoPSIyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiIGNsYXNzPSJsdWNpZGUgbHVjaWRlLWZpbGUiPjxwYXRoIGQ9Ik0xNSAySDZhMiAyIDAgMCAwLTIgMnYxNmEyIDIgMCAwIDAgMiAyaDEyYTIgMiAwIDAgMCAyLTJWN1oiLz48cGF0aCBkPSJNMTQgMnY0YTIgMiAwIDAgMCAyIDJoNCIvPjwvc3ZnPg==";
  const folderImage64 = "data:image/png;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9ImN1cnJlbnRDb2xvciIgc3Ryb2tlLXdpZHRoPSIyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiIGNsYXNzPSJsdWNpZGUgbHVjaWRlLWZvbGRlciI+PHBhdGggZD0iTTIwIDIwYTIgMiAwIDAgMCAyLTJWOGEyIDIgMCAwIDAtMi0yaC03LjlhMiAyIDAgMCAxLTEuNjktLjlMOS42IDMuOUEyIDIgMCAwIDAgNy45MyAzSDRhMiAyIDAgMCAwLTIgMnYxM2EyIDIgMCAwIDAgMiAyWiIvPjwvc3ZnPg==";
  if (filepath.endsWith('/') || !filepath.slice(-8).includes('.')) {
    startDrag({ item: [filepath], icon: folderImage64 });
  } else {
    startDrag({ item: [filepath], icon: fileImage64 });
  }
}

async function startDrag(
  options: Options,
  onEvent?: (result: CallbackPayload) => void
): Promise<void> {
  await invoke("start_drag", {
    item: options.item,
    image: options.icon,
    onEventFn: onEvent ? transformCallback(onEvent) : null,
  });
}
