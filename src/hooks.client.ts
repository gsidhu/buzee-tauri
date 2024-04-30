import { init } from '@aptabase/web';
setTimeout(() => {
  init('A-EU-6541465318', { appVersion: '0.1.0' });
  console.log('Aptabase hook initialized');
}, 100);