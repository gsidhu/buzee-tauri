import { init } from '@aptabase/web';
setTimeout(() => {
  init('A-EU-6541465318', { appVersion: '0.2.1' });
  console.log('Aptabase hook initialized');
}, 100);