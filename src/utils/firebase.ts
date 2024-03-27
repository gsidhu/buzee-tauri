// import { initializeApp, type FirebaseApp } from 'firebase/app';
// import { getAnalytics, logEvent, type Analytics } from 'firebase/analytics';

// const firebaseConfig = {
// 	apiKey: 'AIzaSyCpR0S6-LR7wrjmlUfybxvMRXEVRaPu2eQ',
// 	authDomain: 'buzee-96a64.firebaseapp.com',
// 	projectId: 'buzee-96a64',
// 	storageBucket: 'buzee-96a64.appspot.com',
// 	messagingSenderId: '215218002613',
// 	appId: '1:215218002613:web:f9e282aa06bef056683cfc',
// 	measurementId: 'G-6DB6185142'
// };

// let app: FirebaseApp;
// let analytics: Analytics;

// if (typeof window !== 'undefined') {
// 	app = initializeApp(firebaseConfig);
// 	analytics = getAnalytics(app);
// }

type EventData = {
	[key: string]: string | number | boolean;
};

// export function sendEvent(name: string, data?: EventData) {
// 	try {
// 		if (analytics && process.env.NODE_ENV === 'production') {
// 			logEvent(analytics, name, data);
// 		}
// 	} catch (e) {
// 		console.error('failed to log event', e);
// 	}
// }

export function sendEvent(name:string, data?: EventData) {
	console.log("sendevent");
}